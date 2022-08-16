# KUKSA.val VSS handling

## Introduction

KUKSA.val is adapted to use Vehicle Signals Specification as defined by COVESA.
The ambition is to always support the latest released version available at the [COVESA VSS release page](https://github.com/COVESA/vehicle_signal_specification/releases).
In addition older versions may be supported. This folder contains copies of all versions supported.

## Supported VSS versions

* [VSS 3.0](https://github.com/COVESA/vehicle_signal_specification/releases/tag/v3.0)
* [VSS 2.2](https://github.com/COVESA/vehicle_signal_specification/releases/tag/v2.2)
* [VSS 2.1](https://github.com/COVESA/vehicle_signal_specification/releases/tag/v2.1)
* [VSS 2.0](https://github.com/COVESA/vehicle_signal_specification/releases/tag/v2.0)

## Change process

This is the process for introducing support for a new VSS version:

* Copy the new json file to this folder
* Add a new contants to `Vssdatabase_record.h` of the format `vss_X_Y_supported = true`
* Check if KUKSA.val code relying on VSS syntax needs to be updated to manage changes in syntax
    * If needed update related code, make sure that the change only concerns affected versions by using the constant defined above.
* Check if examples needs to be updated due to changed signal names or syntax
* Change build scripts and examples to use the new version as default
    * Search for the old version number and replace where needed
* If needed, adapt or extend test cases to use the new version instead of previous version

### Tests after update

* Run kuksa-val-server unit tests according to [documentation](../../kuksa-val-server/test/unit-test/readme.md)
* Build and run kuksa_databroker using the new VSS file according to [documentation](../../kuksa_databroker/README.md)
    * e.g. `cargo run --bin databroker -- --metadata ../data/vss-core/vss_rel_3.0.json`
