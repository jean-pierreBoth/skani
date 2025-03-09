# skani - accurate, fast nucleotide identity calculation for MAGs, genomes, and databases

**This is a version forked from (skani)[https://github.com/bluenote-1577/skani] to update dependencies (needed by [https://github.com/jean-pierreBoth/gsearch])**

## Introduction

**skani** is a program for calculating **average nucleotide identity** (ANI) and **aligned fraction** (AF) for DNA sequences (contigs/MAGs/genomes) and ANI > ~80%.

skani uses an approximate mapping method without base-level alignment to get ANI. It is magnitudes faster than BLAST-based methods and almost as accurate. skani offers:

1. **Accurate ANI calculations for MAGs**. skani is accurate for incomplete and medium-quality metagenome-assembled genomes (MAGs). Pure sketching methods (e.g. Mash) may underestimate ANI for incomplete MAGs.

2. **Aligned fraction results**. skani outputs the fraction of genome aligned. 

3. **Fast computations**. Indexing/sketching is ~ 3x faster than Mash, and querying is about 25x faster than FastANI (but slower than Mash). 

4. **Efficient database search**. Querying a genome against a preprocessed database of >65000 prokaryotic genomes takes seconds with a single processor and ~6 GB of RAM. Constructing a database from genome sequences takes minutes to an hour.

##  Updates

### v0.2.2 - 2024-07-04

* Added the `--small-genomes` preset that is an alias for `-c 30 -m 200 --faster-small`
* Fixed some bugs

See the [CHANGELOG](https://github.com/bluenote-1577/skani/blob/main/CHANGELOG.md) for the skani's full versioning history. 

##  Install

#### Option 1: Build from source

Requirements:
1. [rust](https://www.rust-lang.org/tools/install) programming language and associated tools such as cargo are required and assumed to be in PATH.
2. A c compiler (e.g. GCC)
3. make

Building takes a few minutes (depending on # of cores).

```sh
git clone https://github.com/bluenote-1577/skani
cd skani

# If default rust install directory is ~/.cargo
cargo install --path . --root ~/.cargo
skani dist refs/e.coli-EC590.fasta refs/e.coli-K12.fasta

# If ~/.cargo doesn't exist use below commands instead
#cargo build --release
#./target/release/skani dist refs/e.coli-EC590.fasta refs/e.coli-K12.fasta
```

See the [Releases](https://github.com/bluenote-1577/skani/releases) page for obtaining specific versions of skani.

#### Option 2: Conda (source version: 0.2.1)
[![Anaconda-Server Badge](https://anaconda.org/bioconda/skani/badges/version.svg)](https://anaconda.org/bioconda/skani)
[![Anaconda-Server Badge](https://anaconda.org/bioconda/skani/badges/latest_release_date.svg)](https://anaconda.org/bioconda/skani)
```sh
conda install -c bioconda skani
```

#### Option 3: Pre-built x86-64 linux statically compiled executable

We offer a pre-built statically compiled executable for x86-64 Linux systems. That is, if you're on an x86-64 Linux system, you can just download the binary and run it without installing anything. 

For using the latest version of skani: 

```sh
wget https://github.com/bluenote-1577/skani/releases/download/latest/skani
chmod +x skani
./skani -h
```

**Important**: the binary runs slightly slower (3-10%) most of the time, but it can be drastically slower on some tasks. 

## Quick start

```sh
# compare two genomes for ANI. skani is symmetric, so order does not affect ANI
skani dist genome1.fa genome2.fa 
skani dist genome2.fa genome1.fa 

# compare multiple genomes; all options take -t for multi-threading.
skani dist -t 3 -q query1.fa query2.fa -r reference1.fa reference2.fa -o all-to-all_results.txt

# compare individual fasta records (e.g. contigs)
skani dist --qi -q assembly1.fa --ri -r assembly2.fa  

# construct database and do memory-efficient search
skani sketch genomes_to_search/* -o database
skani search query1.fa query2.fa ... -d database

# use sketch from "skani sketch" output as drop-in replacement
skani dist database/query.fa.sketch database/ref.fa.sketch

# construct similarity matrix/edge list for all genomes in folder
skani triangle genome_folder/* > skani_ani_matrix.txt
skani triangle genome_folder/* -E > skani_ani_edge_list.txt

# we provide a script in this repository for clustering/visualizing distance matrices.
# requires python3, seaborn, scipy/numpy, and matplotlib.
python scripts/clustermap_triangle.py skani_ani_matrix.txt 

```

## Tutorials and manuals

### [skani basic usage information](https://github.com/bluenote-1577/skani/wiki/skani-basic-usage-guide)

For more information about using the specific skani subcommands, see the [guide linked above](https://github.com/bluenote-1577/skani/wiki/skani-basic-usage-guide). 

### skani tutorials

1. #### [Tutorial: setting up the GTDB prokaryotic genome database to search against](https://github.com/bluenote-1577/skani/wiki/Tutorial:-setting-up-the-GTDB-genome-database-to-search-against)
2. #### [Tutorial: classifying entire assemblies against > 85,000 genomes in under 2 minutes](https://github.com/bluenote-1577/skani/wiki/Tutorial:-classifying-entire-assemblies-(MAGs-or-contigs)-against-85,000-genomes-in-under-2-minutes)
3. #### [Tutorial: strain-level clustering of MAGs using skani, and why Mash/FastANI have issues](https://github.com/bluenote-1577/skani/wiki/Tutorial:-strain-and-species-level-clustering-of-MAGs-with-skani-triangle)

### [skani cookbook](https://github.com/bluenote-1577/skani/wiki/skani-cookbook)

Some common use cases and parameter settings are outlined in the cookbook. 

### [Pre-sketched databases for searching](https://github.com/bluenote-1577/skani/wiki/Pre%E2%80%90sketched-databases)

Pre-sketched databases can be downloaded and quickly searched against. GTDB-R214 is currently supported. 

### [skani advanced usage information](https://github.com/bluenote-1577/skani/wiki/skani-advanced-usage-guide)

See the advanced usage guide linked above for more information about topics such as:

* optimizing sensitivity/speed of skani
* optimizing skani for long-reads or contigs
* making skani for memory efficient for huge data sets

## Output

If the resulting aligned fraction for the two genomes is < 15%, no output is given. 

**In practice, this means that only results with > ~82% ANI are reliably output** (with default parameters). See the [skani advanced usage guide](https://github.com/bluenote-1577/skani/wiki/skani-advanced-usage-guide) for information on how to compare lower ANI genomes. 

The default output for `search` and `dist` looks like
```
Ref_file	Query_file	ANI	Align_fraction_ref	Align_fraction_query	Ref_name	Query_name
refs/e.coli-EC590.fasta	refs/e.coli-K12.fasta	99.39	93.95	93.37	NZ_CP016182.2 Escherichia coli strain EC590 chromosome, complete genome	NC_007779.1 Escherichia coli str. K-12 substr. W3110, complete sequence
```
- Ref_file: the filename of the reference.
- Query_file: the filename of the query.
- ANI: the ANI.
- Aligned_fraction_query/reference: fraction of query/reference covered by alignments.
- Ref/Query_name: the id of the first record in the reference/query file.

The order of results is dependent on the command and not guaranteed to be deterministic when > 5000 query genomes are present. `dist` and `search` try to place the highest ANI results first. 

## Citation

Jim Shaw and Yun William Yu. Fast and robust metagenomic sequence comparison through sparse chaining with skani. Nature Methods (2023). https://doi.org/10.1038/s41592-023-02018-3

## Feature requests, issues

skani is actively being developed by me ([Jim Shaw](https://jim-shaw-bluenote.github.io/)). I'm more than happy to accommodate simple feature requests (different types of outputs, etc). Feel free to open an issue with your feature request on the GitHub repository. If you catch any bugs, please open an issue or e-mail me (e-mail on my website). 

## Calling skani from rust or python

### Rust API

If you're interested in using skani as a rust library, check out the minimal example here: https://github.com/bluenote-1577/skani-lib-example. The documentation is currently minimal (https://docs.rs/skani/0.1.0/skani/) and I guarantee no API stability. 

### Python bindings 

If you're interested in calling skani from python, see the [pyskani](https://github.com/althonos/pyskani) python interface and bindings to skani written by [Martin Larralde](https://github.com/althonos). Note: I am not personally involved in the pyskani project and do not offer guarantees on the correctness of the outputs. 
