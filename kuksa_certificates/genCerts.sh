#!/bin/bash


genCAKey() {
    openssl genrsa -out CA.key 2048
}


genCACert() {
    openssl req -key CA.key -new -out CA.csr -subj "/C=CA/ST=Ontario/L=Ottawa/O=Eclipse.org Foundation, Inc./CN=localhost-ca/emailAddress=kuksa-dev@eclipse.org"
    openssl x509 -signkey CA.key -in CA.csr -req -days 3650 -out CA.pem
}

genKey() {
    openssl genrsa -out $1.key 2048
}

# This method (and how it is called) contains some hacks to pass name verification
# CN is called as per argument
# We also include that as subjectAltName
# databroker-cli currently expects certificate for Databroker to be called "Server"
# We add localhost and 127.0.0.1 as subjectAltName as they are common names used in test
# (But many TLS-client implementations ignore IP:XXX.XXX.XXX.XXX alt names)
genCert() {
    openssl req -new -key $1.key -out $1.csr -passin pass:"temp" -subj "/C=CA/ST=Ontario/L=Ottawa/O=Eclipse.org Foundation, Inc./CN=$1/emailAddress=kuksa-dev@eclipse.org"
    openssl x509 -req -in $1.csr -extfile <(printf "subjectAltName=DNS:$1, DNS:localhost, IP:127.0.0.1") -CA CA.pem -CAkey CA.key -CAcreateserial -days 365 -out $1.pem
    openssl verify -CAfile CA.pem $1.pem
}

set -e
# Check if the CA is available, else make CA certificates
if [ -f "CA.key" ]; then
    echo "Existing CA.key will be used"
else
    echo "No CA.key found, will generate new key"
    genCAKey
    rm -f CA.pem
    echo ""
fi

# Check if the CA.pem is available, else generate a new CA.pem
if [ -f "CA.pem" ]; then
    echo "CA.pem will not be regenerated"
else
    echo "No CA.pem found, will generate new CA.pem"
    genCACert
    echo ""
fi


for i in Server Client;
do
    if [ -f $i.key ]; then
        echo "Existing $i.key will be used"
    else
        echo "No $i.key found, will generate new key"
        genKey $i
    fi
    echo ""
    echo "Generating $i.pem"
    genCert $i
    echo ""
done

