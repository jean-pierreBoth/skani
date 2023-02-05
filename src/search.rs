use crate::chain;
use crate::regression;
use crate::file_io;
use crate::params::*;
use crate::screen;
use crate::parse;
use crate::types::*;
use fxhash::FxHashMap;
use log::*;
use rayon::prelude::*;
use std::path::Path;
use std::sync::Mutex;
use std::sync::RwLock;
use std::time::Instant;

pub fn search(command_params: CommandParams) {
    let now = Instant::now();
    info!("Searching...");
    let mut ref_marker_file = "";
    for file in command_params.ref_files.iter() {
        if !file.contains(".sketch") && !file.contains("marker") {
            warn!(
                "{} does not have .sketch as an extension; skipping file",
                file
            );
        } else if file.contains("markers.bin") {
            ref_marker_file = file;
        }
    }

    if ref_marker_file.is_empty() {
        //error!("No sketch files found in the folder. Sketch files must be generated by `skani sketch` and have the .sketch extension.");
        error!("markers.bin not found in the folder. Ensure that the folder was generated by `skani sketch`.");
        std::process::exit(1)
    }

    let ref_sketches;
    let sketch_params;
    (sketch_params, ref_sketches) = file_io::marker_sketches_from_marker_file(ref_marker_file);
    let screen_val;
    if command_params.screen_val == 0. {
        if sketch_params.use_aa {
            screen_val = SEARCH_AAI_CUTOFF_DEFAULT;
        } else {
            screen_val = SEARCH_ANI_CUTOFF_DEFAULT;
        }
    } else {
        screen_val = command_params.screen_val;
    }

    info!("Loading markers time: {}", now.elapsed().as_secs_f32());
    let kmer_to_sketch;
    if command_params.screen {
        let now = Instant::now();
        info!("Full index option detected; generating marker hash table");
        kmer_to_sketch = screen::kmer_to_sketch_from_refs(&ref_sketches);
        info!("Full indexing time: {}", now.elapsed().as_secs_f32());
    } else {
        kmer_to_sketch = KmerToSketch::default();
    }
    let ref_sketches_used: RwLock<FxHashMap<_, _>> = RwLock::new(FxHashMap::default());

    let now = Instant::now();
    //assert!(ref_sketches.len() == ref_marker_files.len());
    let anis: Mutex<Vec<AniEstResult>> = Mutex::new(vec![]);
    let counter: Mutex<usize> = Mutex::new(0);
    let folder = Path::new(&ref_marker_file).parent().unwrap();
    for query_file in command_params.query_files.iter() {
        let query_params;
        let query_sketches;
        if command_params.queries_are_sketch {
            (query_params, query_sketches) =
                file_io::sketches_from_sketch(&vec![query_file.clone()]);
            if query_params != sketch_params && !query_file.contains("markers.bin") {
                warn!("Query sketch parameters for {} not equal to reference sketch parameters; no ANI calculated", query_file);
            }
        } else if command_params.individual_contig_q {
            query_sketches = file_io::fastx_to_multiple_sketch_rewrite(
                &vec![query_file.clone()],
                &sketch_params,
                true,
            );
        } else {
            query_sketches =
                file_io::fastx_to_sketches(&vec![query_file.clone()], &sketch_params, true);
        }

        if !query_sketches.is_empty() {
            let is = 0..query_sketches.len();
            is.into_par_iter().for_each(|i| {
                let query_sketch = &query_sketches[i];
                let refs_to_try;
                if !command_params.screen {
                    let refs_to_try_mutex: Mutex<Vec<&String>> = Mutex::new(vec![]);
                    let js = 0..ref_sketches.len();
                    js.into_par_iter().for_each(|j| {
                        let ref_sketch = &ref_sketches[j];
                        if chain::check_markers_quickly(query_sketch, ref_sketch, screen_val) {
                            let mut lock = refs_to_try_mutex.lock().unwrap();
                            lock.push(&ref_sketches[j].file_name);
                        }
                    });
                    refs_to_try = refs_to_try_mutex.into_inner().unwrap();
                } else {
                    refs_to_try = screen::screen_refs_filenames(
                        screen_val,
                        &kmer_to_sketch,
                        query_sketch,
                        &sketch_params,
                        &ref_sketches,
                    );
                }
                debug!("Refs to try {}", refs_to_try.len());
                let js = 0..refs_to_try.len();
                js.into_par_iter().for_each(|j| {
                    let original_file = &refs_to_try[j];
                    let ref_sketch;
                    if !command_params.keep_refs {
                        let sketch_file = folder.join(
                            Path::new(&format!("{}.sketch", original_file))
                                .file_name()
                                .unwrap(),
                        );
                        let (_sketch_params_ref, ref_sketch_new) = file_io::sketches_from_sketch(
                            &vec![sketch_file.to_str().unwrap().to_string()],
                        );
                        ref_sketch = ref_sketch_new;
                        let map_params = chain::map_params_from_sketch(
                            &ref_sketch[0],
                            sketch_params.use_aa,
                            &command_params,
                        );
                        let ani_res;
                        if map_params != MapParams::default() {
                            ani_res = chain::chain_seeds(&ref_sketch[0], query_sketch, map_params);
                        } else {
                            ani_res = AniEstResult::default();
                        }
                        if ani_res.ani > 0.5 {
                            let mut locked = anis.lock().unwrap();
                            locked.push(ani_res);
                        }
                    } else {
                        let mut contains = false;
                        {
                            let read_table = ref_sketches_used.read().unwrap();
                            if read_table.contains_key(original_file) {
                                contains = true;
                            }
                        }
                        if contains {
                            let read_table = ref_sketches_used.read().unwrap();
                            let ref_sketch: &Vec<_> = &read_table[original_file];
                            let map_params = chain::map_params_from_sketch(
                                &ref_sketch[0],
                                sketch_params.use_aa,
                                &command_params,
                            );
                            let ani_res;
                            if map_params != MapParams::default() {
                                ani_res =
                                    chain::chain_seeds(&ref_sketch[0], query_sketch, map_params);
                            } else {
                                ani_res = AniEstResult::default();
                            }
                            if ani_res.ani > 0.5 {
                                let mut locked = anis.lock().unwrap();
                                locked.push(ani_res);
                            }
                        } else {
                            let sketch_file = folder.join(
                                Path::new(&format!("{}.sketch", original_file))
                                    .file_name()
                                    .unwrap(),
                            );
                            let (_sketch_params_ref, ref_sketch) = file_io::sketches_from_sketch(
                                &vec![sketch_file.to_str().unwrap().to_string()],
                            );

                            let map_params = chain::map_params_from_sketch(
                                &ref_sketch[0],
                                sketch_params.use_aa,
                                &command_params,
                            );
                            let ani_res;
                            if map_params != MapParams::default() {
                                ani_res =
                                    chain::chain_seeds(&ref_sketch[0], query_sketch, map_params);
                            } else {
                                ani_res = AniEstResult::default();
                            }

                            {
                                let mut write_table = ref_sketches_used.write().unwrap();
                                write_table.insert(original_file.clone(), ref_sketch);
                            }

                            if ani_res.ani > 0.5 {
                                {
                                    let mut locked = anis.lock().unwrap();
                                    locked.push(ani_res);
                                }
                            }
                        }
                    }
                });

                let c;
                {
                    let mut locked = counter.lock().unwrap();
                    *locked += 1;
                    c = *locked
                }
                if c % 100 == 0 && c != 0 {
                    info!("{} query sequences processed.", c);
                }
            });
        }
    }
    if command_params.keep_refs{
        info!("{} references kept in memory for --keep-refs", ref_sketches_used.read().unwrap().len());
    }
    let mut anis = anis.into_inner().unwrap();
    let learned_ani;
    if !command_params.learned_ani_cmd{
        learned_ani = parse::use_learned_ani(sketch_params.c, command_params.individual_contig_q, command_params.individual_contig_r, command_params.robust, command_params.median);
    }
    else{
        learned_ani = command_params.learned_ani;
    }
    let model_opt = regression::get_model(sketch_params.c, learned_ani);
    if model_opt.is_some(){
        info!("Learned ANI mode detected. ANI will be adjusted according to a pre-trained regression model. Use --no-learned-ani to disable.");
        let model = model_opt.as_ref().unwrap();
        for ani in anis.iter_mut(){
            regression::predict_from_ani_res(ani, &model);
        }
    }
    file_io::write_query_ref_list(
        &anis,
        &command_params.out_file_name,
        command_params.max_results,
        sketch_params.use_aa,
        command_params.est_ci,
        command_params.detailed_out,
    );
    info!("Searching time: {}", now.elapsed().as_secs_f32());
}
