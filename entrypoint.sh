#!/bin/sh
set -e

echo "preparation de diesel..."
diesel setup

echo "Appliquer les migrations..."
diesel migration run

echo "Compilation du projet..."
cargo build --release

echo "Lancement de l'API..."
exec ./target/release/projet_picsou_api
