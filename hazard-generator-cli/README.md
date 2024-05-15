# hazard-generator-cli

This tool makes use of the `hazard-generator` library to generates hazards APIs starting from a JSON ontology.

## Building

Use this command to build the tool:

```console
cargo build 
```

## Usage

Run `hazard-generator-cli` on a project with the following command:

```console
hazard-generator-cli [OPTIONS] -p <ONTOLOGY_PATH> -t <TEMPLATE> -o <OUTPUT_PATH> 
```

To see the list of supported options, run:

```console
hazard-generator-cli --help
```

### Ontology Path

To specify the path of the ontology file, use `--ontology_path` or `-p`:

```console
hazard-generator-cli -p ontology.jsonld -t <TEMPLATE> -o <OUTPUT_PATH>  
```

### Template

To specify the template name use `--template` or `-t`:

```console
hazard-generator-cli -p <ONTOLOGY_PATH> -t rust -o <OUTPUT_PATH>   
```

### With Risk

To parse the risk score associated to the hazards, use `--with-risk` or `-r`:

```console
hazard-generator-cli -p <ONTOLOGY_PATH> -t rust -o <OUTPUT_PATH> --with-risk   
```

### Output Path

To specify the path of the output directory, use `--output-path` or `-o`:

```console
hazard-generator-cli -p <ONTOLOGY_PATH> -t <TEMPLATE> -o ./
```

### Verbose

To see the paths of the generated templates as they are produced, use `--verbose` or `-v`:

```console
hazard-generator-cli -p <ONTOLOGY_PATH> -t <TEMPLATE> -o <OUTPUT_PATH> -v
```