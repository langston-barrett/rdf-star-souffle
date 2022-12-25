# rdf-star-souffle

rdf-star-souffle is a toolkit for importing and reasoning about 
[RDF-star][rdf-star] data in [Soufflé][souffle]. It consists of a few parts:

- A tool for ingesting N-Triples-star syntax and emitting Soufflé-compatible
  CSV files (`rdf-star-facts`)
- Soufflé types and relations for ingesting RDF-star data
- A Soufflé implementation of an RDFS reasone
- A Soufflé implementation of an OWL reasoner

This project is a work-in-progress. The reasoners are not yet implemented.

<!-- See also: https://github.com/gtfierro/reasonable -->

[rdf-star]: https://www.w3.org/2021/12/rdf-star.html
[souffle]: https://souffle-lang.github.io/index.html