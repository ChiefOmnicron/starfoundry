# StarFoundry

Industry tool for [EVE-Online](https://www.eveonline.com/).

## Services

The project is structured as a monorepo with a couple of services.

### Gateway

Serves as the entry into the infrastructure.

- Routes are configured via a config file
  - Only the configured routes can be called, and require auth per default
- Deals with JWT-Tokens and makes sure the user is allowed to access

### Eve-Gateway

Everything that has to do with the EVE-API, is wrapped by this services and exposed via REST.

- Provides routes for authenticating via EVE-SSO
  - Login of main characters
  - Adding alt characters
  - Logging in a corporation
  - Applications are identified by their Domain and can request different permissions/have different admins/whitelist/blacklist
- Provides an interface to imported [SDE](https://developers.eveonline.com/static-data) data
- Provides an interface for other routes to the [EVE-API](https://developers.eveonline.com/api-explorer)

### Industry

Primary service for serving industry related information.

- Planner for products
  - Takes Stock, Blueprint ME, and structure bonuses into account
  - Additional configuration options
    - Max number of runs per Blueprint
    - Overwrite of Blueprint ME
    - Blacklist products that shouldn't be build
  - Generates a list of jobs and needed materials to fulfill the targeted product
- Project Groups that serve as the basis for default configurations and sharing between others
- Industry-Hub representation of an in-game Industry-Hub with all it's structures and bonuses
- Job-Assignment for working together in a group for a common goal
- Job-Detection auto track started jobs to their project
- Cost tracking of materials, started jobs, and miscellaneous items

### Market

Provides the REST interface for requesting the cost of materials

- Information based on the latest EVE-API data
- Different strategies for providing market data
  - MultiBuy -> handles like the in-game MultiBuy window
  - SmartBuy -> Optimized method that takes more information into account

## Workers

Background workers that fetch data from the EVE-API.

### Eve-Gateway

- Character assets
- Corporation assets
- Character Blueprints
- Corporation Blueprints
- System index
  - Compression of the system index

### Industry

- Character industry jobs
- Corporation industry jobs
- Job-Detection

### Market

- Market information for NPC-Stations
- Market information for Player-Stations
- Market information for a Region
- Public contracts + their content
- Character market orders
- Corporation market orders
- Material prices

### SDE-Parser

- Parsing of several YAML files
  - Blueprints
  - TypeIds
  - Categories
  - Groups
  - Systems
  - Dogma
- Parsing of json files from Hoboleaks
  - Industry Modifier Source
  - Industry Target Filters
- Custom overwrites of items and blueprints
- All data is modified to serve the application best

## Applications

Different applications are build on-top of the underlying platform.

### Industry

Primary [Industry](https://industry.alpha.starfoundry.space) interface, and the primary application that kicked this project off.

### Industry-Hub

Focuses on [Industry-Hubs](https://industry-hubs.starfoundry.space). Stripped down interface of the industry application.
