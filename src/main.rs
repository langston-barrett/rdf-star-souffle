// TODO: Reify all quoted triples

use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use rio_api::model::{Literal, NamedNode, Subject, Term, Triple};
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError};

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg()]
    file: PathBuf,
}

fn quoted_triple(t: &Triple<'_>) -> String {
    format!(
        "\"{} {} {}\"",
        subject(&t.subject),
        predicate(&t.predicate),
        object(&t.object)
    )
}

fn subject(s: &Subject<'_>) -> String {
    match s {
        Subject::NamedNode(n) => n.iri.to_string(),
        Subject::BlankNode(_) => todo!(),
        Subject::Triple(t) => quoted_triple(t),
    }
}

fn predicate(n: &NamedNode<'_>) -> String {
    n.iri.to_string()
}

fn object(t: &Term<'_>) -> String {
    match t {
        Term::NamedNode(n) => n.iri.to_string(),
        Term::BlankNode(_) => panic!("Named nodes not supported"),
        Term::Literal(Literal::Simple { value }) => value.to_string(),
        Term::Literal(_) => panic!("Literal not supported"),
        Term::Triple(t) => quoted_triple(t),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let content = fs::read(args.file)?;
    let mut out = csv::Writer::from_writer(std::io::stdout());
    NTriplesParser::new(content.as_ref()).parse_all(&mut |t| {
        out.write_record(&[
            subject(&t.subject),
            predicate(&t.predicate),
            object(&t.object),
        ])
        .unwrap();
        Ok::<(), TurtleError>(())
    })?;
    Ok(())
}
