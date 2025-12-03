# Use cases Politieke groepering

## Politieke groepering maakt de set benodigde documenten (vlieger)

__Niveau:__ Hoog-over, vlieger, 🪁

### Hoofdscenario en uitbreidingen

__Hoofdscenario:__

1. [Politieke groepering zet alle algemene lijstgegevens in de applicatie.](#politieke-groepering-zet-alle-algemene-lijstgegevens-in-de-applicatie-zee)
2. [Politieke groepering zet alle personen in de applicatie.](#politieke-groepering-zet-alle-personen-in-de-applicatie-zee)
3. [Politieke groepering maakt de kandidatenlijsten.](#politieke-groepering-maakt-de-kandidatenlijsten-zee)
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

## Politieke groepering maakt de kandidatenlijsten (zee) 

*Nog te bespreken met Marlon en Grietje*

__Niveau:__ Gebruikersdoel, zee, 🌊

__Hoofdscenario:__

1. Politieke groepering selecteert dat ze meedoen met één lijst voor alle gebieden*
2. Politieke groepering vult de lijst met de eerder ingevulde personenset
3. Politieke groepering zet de kandidaten in de goede volgorde


__Uitbreidingen__:
1a. Politieke groepering doet mee met meerdere lijsten
&emsp; 1a1. Politieke groepering selecteert gebied(en) waar de lijst geldig is
2a. Politieke groepering voegt een kandidaat toe die nog niet in de personenset staat
4. Politieke groepering doet mee met meerdere lijsten
&emsp; 4a1. Politieke groepering selecteerd nieuwe gebieden waar de lijst geldig is
&emsp; 4a2. Politieke groepering selecteert een lijst die als basis dient 
&emsp; &emsp; 4a2a. Politieke groepering vult de lijst vanaf 0 met de ingevulde personenset
&emsp; &emsp; 4a2b. Politieke groepering zet de kandidaten in de goede volgorde
&emsp; 4a3. Politieke groepering pas de lijst aan door kandidaten en de volgorde te wijzigen

## Politieke groepering machtigt de lijstinleveraar (zee)

__Niveau:__ Gebruikersdoel, zee, 🌊
