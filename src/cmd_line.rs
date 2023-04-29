pub const MIN_ALIGN_FRAC: &str = "min aligned frac";
pub const CMD_MIN_ALIGN_FRAC: &str = "min-af";
pub const H_MIN_ALIGN_FRAC: &str = "Only output ANI values where one genome has aligned fraction > than this value.\t[default: 15]";

pub const IND_CTG_QRY: &str = "individual contig query";
pub const CMD_IND_CTG_QRY: &str = "qi";
pub const H_IND_CTG_QRY: &str = "Use individual sequences for the QUERY in a multi-line fasta.";

pub const IND_CTG_REF: &str = "individual contig ref";
pub const CMD_IND_CTG_REF: &str = "ri";
pub const H_IND_CTG_REF: &str = "Use individual sequences for the REFERENCE in a multi-line fasta.";

pub const NO_FULL_INDEX: &str = "no marker index";
pub const CMD_NO_FULL_INDEX: &str = "no-marker-index";
pub const H_NO_FULL_INDEX: &str = "Do not use hash-table inverted index for faster ANI filtering. \t[default: load index if > 100 query files or using the --qi option]";

pub const ROBUST: &str = "robust";
pub const CMD_ROBUST: &str = "robust";
pub const H_ROBUST: &str = "Estimate mean after trim off 10%/90% quantiles.";

pub const FULL_MAT: &str = "full-matrix";
pub const CMD_FULL_MAT: &str = "full-matrix";
pub const H_FULL_MAT: &str = "Output full matrix instead of lower-triangular matrix.";

pub const KEEP_REFS: &str = "keep-refs";
pub const CMD_KEEP_REFS: &str = "keep-refs";
pub const H_KEEP_REFS: &str = "Keep reference sketches in memory if the sketch passes the marker filter. Takes more memory but is much faster when querying many similar sequences.";

pub const C_FACTOR: &str = "c";
pub const CMD_C_FACTOR: &str = "c";
pub const H_C_FACTOR: &str = "Compression factor (k-mer subsampling rate).\t[default: 125]";

pub const H_SCREEN: &str = "Screen out pairs with < % identity using k-mer sketching.\t[default: 80]";

pub const CONF_INTERVAL: &str = "ci";
pub const CMD_CONF_INTERVAL: &str = "ci";
pub const H_CONF_INTERVAL: &str = "Output [5%,95%] ANI confidence intervals using percentile bootstrap on the putative ANI distribution.";
pub const H_CONF_INTERVAL_TRI: &str = "Output [5%,95%] ANI confidence intervals using percentile bootstrap on the putative ANI distribution. Only works with --sparse or -E.";

pub const LEARNED_ANI: &str = "learned-regression-model";
pub const CMD_LEARNED_ANI : &str = "learned-ani";
pub const H_LEARNED_ANI: &str = "Use ANI prediction with a regression model trained on MAG data. \t[default: on if c >= 70, >= 150,000 bases aligned]";

pub const NO_LEARNED_ANI: &str = "no-learned-regression-model";
pub const CMD_NO_LEARNED_ANI : &str = "no-learned-ani";
pub const H_NO_LEARNED_ANI: &str = "Disable regression model for ANI prediction.\t[default: disabled unless c >= 70, >= 150,000 bases aligned]";

pub const MODE_SLOW: &str = "slow";
pub const CMD_MODE_SLOW : &str = "slow";
pub const H_MODE_SLOW : &str = "Slower skani mode; 4x slower and more memory. Gives much more accurate AF for distant genomes. More accurate ANI for VERY fragmented assemblies (< 3kb N50), but less accurate ANI otherwise. Alias for -c 30.";

pub const MODE_FAST: &str = "fast";
pub const CMD_MODE_FAST : &str = "fast";
pub const H_MODE_FAST : &str = "Faster skani mode; 2x faster and less memory. Less accurate AF and less accurate ANI for distant genomes, but works ok for high N50 and > 95% ANI. Alias for -c 200.";

pub const MODE_MEDIUM: &str = "medium";
pub const CMD_MODE_MEDIUM : &str = "medium";
pub const H_MODE_MEDIUM: &str = "Medium skani mode; 2x slower and more memory. More accurate AF and more accurate ANI for moderately fragmented assemblies (< 10kb N50). Alias for -c 70.";

pub const MARKER_C: &str = "marker_c";
pub const CMD_MARKER_C: char = 'm';
pub const H_MARKER_C: &str = "Marker k-mer compression factor. Markers are used for filtering. Consider decreasing if genome is small or mapping individual small contigs. \t[default: 1000]";

pub const DETAIL_OUT: &str = "detailed";
pub const CMD_DETAIL_OUT: &str = "detailed";
pub const H_DETAIL_OUT: &str = "Print additional info including contig N50s and more.";

pub const DISTANCE_OUT: &str = "distance";
pub const CMD_DISTANCE_OUT: &str = "distance";
pub const H_DISTANCE_OUT: &str = "Output 100 - ANI instead of ANI, creating a distance instead of a similarity matrix.";


