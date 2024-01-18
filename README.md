# UDA Results Extractor

# Required envs

The following env vars are required:

| Variable     | Definition                                               | Example             |
|--------------|----------------------------------------------------------|---------------------|
| UDA_USERNAME | Username of the account the data will be downloaded with | example@example.com |
| UDA_PASSWORD | Password of the account the data will be downloaded with | Th1s1sAPa55w0rd     |
| CONVENTIONS  | comma-separated list of conventions to extract from UDA  | unicon2020,cfm2023  |

These vars can be passed to the app through env
var (`UDA_USERNAME=<> UDA_PASSWORD=<> CONVENTIONS=<> uda-results-extractor`)
or using a `.env` file. The latter should be located in the execution folder.

# Required rights

In order to be able to export data from a convention, the user should have the following rights:

- event_planner
- competition_admin

If they don't have the rights, then the extractor will fail for that particular convention.
The other conventions will be handled though.

# Class diagram

```mermaid
classDiagram
    direction TD

    class Person {
        name: PersonName
        registrations: Vec~Registration~
    }

    class PersonName {
        name: String
    %% name_parts is for equality checks
        name_parts: Vec<String>
    }

    class Registration {
        competitor: Competitor
        convention: Convention
        competitions: HashMap~Competition, CompetitionResult~
    }

    class Convention {
        tag: String
        name: String
    }

    class Competition {
        name: String
    }

    class Competitor {
        <<enumeration>>
        Team: Team
        IndividualCompetitor: IndividualCompetitor
        UnknownIndividualCompetitor: UnknownIndividualCompetitor
    }

    class Team {
        name: String
        members: Vec~Competitor~
    }

    class IndividualCompetitor {
        id: u16
        name: PersonName
    %% gender and age depends on the competition, so can't be added to `Person`
        gender: Gender
        age: u8
    }

    class UnknownIndividualCompetitor {
        id: u16
    }

%%    note for Gender "Should be one of [Male, Female]"
    class Gender {
        gender: String
    }

    class ResultValue {
        <<enumeration>>
        Empty,
        Time: Duration,
        Points: f32,
        Distance: u16,
        Custom: String,
    }

%%    note for Place "If disqualified, should be one of [DNF, DQ]"
    class Place {
        <<enumeration>>
        Rank: u16,
        Disqualified: String,
    }

%%    note for ResultType "Should be one of [AgeGroup, Overall]"
    class ResultType {
        result_type: String
    }

    class AgeGroup {
    %% An age group may be in fact a gathering of multiple age groups
        groups_name: Vec~String~
    }

    class CompetitionResult {
        place: Place,
        result_type: ResultType,
        result: ResultValue,
        details: String,
        age_group: AgeGroup
    }

    Person *-- Registration
    Registration o-- Convention
    Registration o-- Competition
    Registration o-- Competitor
    Registration *-- CompetitionResult
    Person *-- PersonName
    Competitor --> Team
    Competitor --> IndividualCompetitor
    Competitor --> UnknownIndividualCompetitor
    Team o-- Competitor
    IndividualCompetitor <-- Gender
    CompetitionResult *-- Place
    CompetitionResult *-- ResultType
    CompetitionResult *-- ResultValue
    CompetitionResult *-- AgeGroup
```