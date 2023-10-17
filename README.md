# SDF utils CLI tools

Small collection of tools to process sdf files.

## partition

Split single large sdf files into smaller partitions

Example, splitting chembl into 24 (almost) equal chunks:

`cargo run --bin sdf-partition -- /data/chembl/chembl_33.sdf 24`