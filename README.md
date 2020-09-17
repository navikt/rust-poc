# rust-poc

Forhåpentligvis en grei proof of concept som beviser at Rust kan brukes til utvikling av tjenester i Nav!

## Så langt

- [x] En webapp som svarer på vanlige HTTP-requests
    - Bygger på [Rocket](https://rocket.rs).
- [ ] Deployment til NAIS
- [ ] Prometheus
- [ ] Logging
- [ ] En kobling til database
- [ ] En kafka-lytter som lytter på et eller annet
- [ ] En kafka-produsent som produserer et eller annet

### Kjekt å ha

- [ ] Benchmarks!

## Hvorfor?

Idéen er først og fremst å lage noe som realistisk sett kunne dekket et behov i Nav. Dette er best demonstrert ved å benytte de mest prominente tjenestene som blir benyttet i dag, samt å strebe etter lettleselig kode som hvem enn med interesse kan lese og forstå.

Rust dekker det aller meste av pitfalls når det gjelder i minne og threading ved hjelp av statisk analyse av koden når det kompileres. Dette resulterer i at man har tilgang til lavtliggende kode uten at man må være redd for farlige minneoperasjoner og generelle sideffekter.

Det har generelt sett ikke blitt testet å benytte Rust for å skrive applikasjoner i Nav, forutenom noen plattformverktøy ([`deployment-cli`](https://github.com/navikt/deployment-cli), [autoforward](https://github.com/navikt/autoforward)), samt én applikasjonen som treffer noen brukere av nav.no ([tekster-rs](https://github.com/navikt/tekster-rs)). Samtidig er det en liten gruppe folk i Nav som har ønske om å utforske denne muligheten mer, og i den sammenheng er dette repoet laget for å kunne prøve det ut med litt frie tøyler!