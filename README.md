## MS-DHCP-TO-ISC-DHCP

A command-line utility for migrating the configuration of Microsoft Server DHCP 2016-2025 to the ISC DHCP format, written in Rust!

## Introduction

Currently, only IPv4 configuration migration is supported, but IPv6 may also appear in the future.

## CLI Parameters

Command line interface 'MS-DHCP-TO-ISC-DHCP' implements the following interaction parameters:

`--microsoft-filepath` - Required. Specifies the path to the Microsoft Server DHCP configuration file.

`--config-filepath` - Optional. Specifies the path to the ISC DHCP configuration file. If not specified, the file 'dhcpd.conf' will be created in the directory of the utility call.

`--with-transliterate` - Optional. Performs the transliteration process (GOST 7.79-2000. System 'B'./ISO 9:1995) before starting migration for Microsoft Server DHCP configuration.

## Migrated successfully

The migration utility allows you to successfully migrate the following parameters:

|Microsoft configuration part|ISC configuration part|
|--|--|
|Classes (including vendor/user)|Classes|
|Option Definitions (including vendor options)|Option spaces, Option definitions|
|Options (including vendor options)|Options|
|Policies|Classes, Option spaces, Options|
|Filters|Classes, Subclasses, Hosts|
|Scopes|Subnets, Classes, Option spaces, Options, Hosts|

## NOT Migrated

The migration utility cannot migrate the following parameters:

|Microsoft configuration part|Why|
|--|--|
|Options with User Class|Option definitions with user classes are not exported from the configuration, and their type cannot be calculated|
|Policy IP Ranges|It is not possible to cover all migration scenarios correctly. Perhaps support will appear in the future|
|DNS Settings|Are not exported to the configuration|
|Leases|There is no need for migration, these parameters will appear when clients request addresses|
|High Availability|Linking the "MS - ISC" format servers is impossible, migration does not make sense|
|Superscopes|Migration is possible and will appear in the future|
|Multicast scopes|Are not exported to the configuration|
|Policy "RelayAgent" condition|There is no direct equivalent in the ISC configuration|

## License

[GPLv3](https://www.gnu.org/licenses/gpl-3.0.html)
