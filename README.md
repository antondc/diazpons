# Web for diazpons.es

Web client for diazpons.

## Install and run

### Install dependencies

    npm i
    cargo build

### Install runtime environment, dependencies and run

    build::all

## Create certificate

### Generate ssl certificates with Subject Alt Names on OSX

### Create `ssl.conf` file

      [ req ]
      default_bits       = 4096
      distinguished_name = req_distinguished_name
      req_extensions     = req_ext

      [ req_distinguished_name ]
      countryName                 = ES
      countryName_default         = MA
      stateOrProvinceName         = MA
      stateOrProvinceName_default = MA
      localityName                = MA
      localityName_default        = MA
      organizationName            = diazpons
      organizationName_default    = diazpons
      commonName                  = diazpons.es
      commonName_max              = 64
      commonName_default          = localhost

      [ req_ext ]
      subjectAltName = @alt_names

      [alt_names]
      DNS.1   = diazpons.es
      DNS.2   = dev.diazpons.es

Create a directory `./ssl` for your project close to server, and place `ssl.conf` within it.
Open this folder.

### Generate a private key

    openssl genrsa -out private.key 4096

### Generate a Certificate Signing Request

    openssl req -new -sha256 \
        -out private.csr \
        -key private.key \
        -config ssl.conf

(You will be asked a serie of questions about your certificate. Answer however you like, but for 'Common name' enter the name of your project, e.g. `my_project`)

### Now check the CSR

    openssl req -text -noout -in private.csr

You should see this:

    `X509v3 Subject Alternative Name: DNS:my-project.site` and
    `Signature Algorithm: sha256WithRSAEncryption`

### Generate the certificate

    openssl x509 -req \
        -sha256 \
        -days 3650 \
        -in private.csr \
        -signkey private.key \
        -out private.crt \
        -extensions req_ext \
        -extfile ssl.conf

### Add the certificate to Mac keychain and trust it

    sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain private.crt

(Alternatively, double click on the certificate file `private.crt` to open Keychain Access. Your project name `my_project` will be listed under the login keychain. Double click it and select 'Always trust' under the 'Trust' section.)

## Set domain

Add domain to `hosts` file:

    ##
    # Host Database
    ##
    127.0.0.1       localhost
    255.255.255.255 broadcasthost
    ::1             localhost
    127.0.0.1       dev.diazpons.es # Added

Add local domain where app is accessed to `ENDPOINT_CLIENTS` at `config.test.json`:

    "ENDPOINT_CLIENTS": ["http://dev.diazpons.es", "https://dev.diazpons.es"],

## Conventions

### Naming conventions

    [I(nterface)][Module][Action][ModuleSubtype][ModuleType]

E. g.:

- `ILinkCreateResponse`
- `UserFollowingDeleteController`
- `LinkGetOneUseCase`
- `ILinkRepo`
- `StateRepo`

## License

The MIT License (MIT)

Copyright (c) 2025 Antonio Díaz

[0][1][2]
