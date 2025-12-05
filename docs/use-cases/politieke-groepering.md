# Use cases Politieke groepering

## Politieke groepering maakt de set benodigde documenten (vlieger)

__Niveau:__ Hoog-over, vlieger, 🪁

### Hoofdscenario en uitbreidingen

__Hoofdscenario:__

1. [Politieke groepering zet alle algemene lijstgegevens in de applicatie.](#politieke-groepering-zet-alle-algemene-lijstgegevens-in-de-applicatie-zee)
2. [Politieke groepering maakt de kandidatenlijsten.](#politieke-groepering-maakt-de-kandidatenlijsten-zee)
3. Politieke groepering machtigt de lijstinleveraar.
4. Kandidaten geven instemming met hun plek op de lijst.
5. De lijstinleveraar stelt vast alle gegevens volledig zijn.
6. De lijstinleveraar downloadt en print alle benodigde documenten.

__Uitbreidingen__:

0. Partij heeft geen zetels in vertegenwoordigend orgaan, dus waarborgsom en ondersteuningsverklaringen.


## Gebruiker politieke groepering logt voor de eerste keer in (zee)

__Niveau:__ Gebruikersdoel, zee, 🌊

__Hoofdscenario:__

1. De gebruiker logt in met e-Herkenning.
2. De gebruiker kiest rol: gemachtigde of lijstinleveraar.


## Politieke groepering zet alle algemene lijstgegevens in de applicatie (zee), art. R (voor Eerste Kamerverkiezing)

__Niveau:__ Gebruikersdoel, zee, 🌊



### Hoofdscenario en uitbreidingen

__Hoofdscenario:__

1. De politieke groepering selecteert het type groepering.
2. De gemachtigde vult de gegevens van de lijstinleveraar in. *de stap is hier logisch als je kijkt naar welke gebruiker het doet, maar moet per lijst te kiezen zijn* **art. R 7 (voor Eerste Kamerverkiezing)**
3. De politieke groepering vult de gegevens van de politieke groepering in: aanduiding, volledige statuaire naam, gemachtigde per groepering. **art. R (voor Eerste Kamerverkiezing)** 
4. De applicatie bepaalt de maximale lengte van de lijst o.b.v. de uitslag van de vorige verkiezing. **art. R 4 (voor Eerste Kamerverkiezing)** 
5. De politieke groepering bevestigt de maximale lijstlengte.

### Open punten

- Check wie welke stap doet.
- Checken of inderdaad voor elke H1 een andere lijstinleveraar

## Politieke groepering maakt de kandidatenlijsten (zee) art. R (voor Eerste Kamerverkiezing)

__Niveau:__ Gebruikersdoel, zee, 🌊

__Hoofdscenario:__

1. Politieke groepering maakt een lijst aan
2. Politieke groepering bevestigt lijstinleveraar **art. R 7 (voor Eerste Kamerverkiezing)**
3. Politieke groepering vinkt aan voor welke gebieden de lijst geldig is **art. R 10 (voor Eerste Kamerverkiezing)**
4. Politieke groepering vult de lijst door personen toe te voegen
5. De lijstinleveraar vult de benodigde personalia ([Personalia kandidaat H1](./data.md#personalia-kandidaat-h1-art-h2-kiesbesluit)) in.
6. De applicatie valideert de ingevoerde gegevens en geeft feedback. **art. R 5 (voor Eerste Kamerverkiezing)**
7. Politieke groepering geeft aan dat de lijst klaar is

Optioneel vervolg: door naar stap 1

__Uitbreidingen__:


- 1a. Politieke groepering selecteert een bestaande lijst als sjabloon
- 2a. Politieke groepering kiest voor deze lijst een andere lijstinleveraar
  - 2a1. De gemachtigde bevestigt lijstinleveraar
- 4a. Poltieke groepering vult de lijst en past de volgorde van de kandidaten aan

## Politieke groepering machtigt de lijstinleveraar (zee)

__Niveau:__ Gebruikersdoel, zee, 🌊
