# Use cases Politieke groepering

## Politieke groepering maakt de set benodigde documenten (vlieger)

__Niveau:__ Hoog-over, vlieger, 🪁

### Hoofdscenario en uitbreidingen

__Hoofdscenario:__

1. [Politieke groepering zet alle algemene lijstgegevens in de applicatie.](#politieke-groepering-zet-alle-algemene-lijstgegevens-in-de-applicatie-zee)
2. [Politieke groepering zet alle personen in de applicatie.](#politieke-groepering-zet-alle-personen-in-de-applicatie-zee)
3. Politieke groepering maakt de kandidatenlijsten.
4. Politieke groepering machtigt de lijstinleveraar.
5. Kandidaten geven instemming met hun plek op de lijst.
6. De lijstinleveraar stelt vast alle gegevens volledig zijn.
7. De lijstinleveraar downloadt en print alle benodigde documenten.

__Uitbreidingen__:

- Partij heeft geen zetels in vertegenwoordigend orgaan, dus waarborgsom en ondersteuningsverklaringen.


## Gebruiker politieke groepering logt voor de eerste keer in (zee)

__Niveau:__ Gebruikersdoel, zee, 🌊

__Hoofdscenario:__

1. De gebruiker logt in met e-Herkenning.
2. De gebruiker kiest rol: gemachtigde of lijstinleveraar.


## Politieke groepering zet alle algemene lijstgegevens in de applicatie (zee)

__Niveau:__ Gebruikersdoel, zee, 🌊

__Pre-condities:__

- De gebruiker is ingelogd in de applicatie.

### Hoofdscenario en uitbreidingen

__Hoofdscenario:__

1. De politieke groepering selecteert het type groepering.
2. De gemachtigde vult de gegevens van de lijstinleveraar in.
3. De politieke groepering vult de gegevens van de politieke groepering in: aanduiding, volledige statuaire naam, gemachtigde per groepering.
4. De applicatie bepaalt de maximale lengte van de lijst o.b.v. de uitslag van de vorige verkiezing.
5. De politieke groepering bevestigt de maximale lijstlengte.

### Open punten

- Check wie welke stap doet.

## Politieke groepering zet alle personen in de applicatie (zee)

__Niveau:__ Gebruikersdoel, zee, 🌊

__Pre-condities:__

- De lijstinleveraar is ingelogd in de applicatie.

### Hoofdscenario en uitbreidingen

__Hoofdscenario:__

1. (per persoon) De lijstinleveraar vult de benodigde personalia ([Personalia kandidaat H1](./data.md#personalia-kandidaat-h1-art-h2-kiesbesluit)) in.
2. (per persoon) De applicatie valideert de ingevoerde gegevens en geeft feedback.
