# Use Cases

## Werkwijze

Deze beschrijving van use cases is gebaseerd op "Writing Effective Use Cases" van Alistair Cockburn (2000). Zie ook zijn meer recente boeken "The Mini-Book on Use Cases: All you need but short!" (2025) en "Unifying User Stories, Use Cases, Story Maps: The power of verbs " (2024).

De belangrijkste ideeën zijn:

- Use cases worden in tekst uitgewerkt, diagrammen (UML) zijn een aanvulling.
- Use cases vormen een boomstructuur waarin elke stap in een use case uitgewerkt kan worden tot een onderliggende use case.
- Use cases moeten niet uitgebreider/formeler/gedetailleerder zijn dan strikt nodig is.

Waarom deze werkwijze nuttig kan zijn:

- De boomstructuur is makkelijk te lezen en te navigeren.
- Het geeft een goed overzicht van stakeholders en hun belangen (daar zijn er veel van).
- Het geeft een goed overzicht van varianten ('uitbreidingen') op het hoofdscenario (het 'main success scenario').
- Het is een beschrijving van buitenaf wat de applicatie(s) moet(en) doen, niet hoe.
- Het faciliteert feedback en reviews door stakeholders.

## Veld van een use case

- titel
- niveau
    - Heel hoog-over (wolk) ☁️
    - Hoog-over (vlieger) 🪁
    - Gebruikersdoel (zee) 🌊
        - 1 persoon, 1 sessie (ca. 2-20 minuten)
        - Dit is het ideale niveau voor use cases.
    - Subfunctie (vis) 🐟
        - Schrijf deze alleen wanneer dit echt nodig is.
    - Te laag (schelp) 🐚
        - Dit niveau is te granulair, gooi ze weg.
- precondities
- Hoofdscenario
    - Dit is het meest eenvoudige succes-scenario.
    - Het scenario bevat 5-9 stappen.
    - Beschrijft de acties van actoren om het doel van de primaire actor te bereiken.
    - Er zijn drie soorten acties (stappen):
        - Interactie tussen twee actoren om een doel te bereiken
        - Een validatie om een stakeholder te beschermen
        - Een internal state change namens een stakeholder
- Uitbreidingen
    - Alternatieve successcenario's of foutscenario's
    - De nummering van een uitbreiding komt overeen met de stap in het hoofdscenario waarvan het een uitbreiding is.
- Open punten:
    - Schrijf hier open punten op die gerelateerd zijn aan deze use case.

