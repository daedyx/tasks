# release/

Hier liegt `catalog.pub`: der **öffentliche** Katalog-Schlüssel, mit dem die Pipeline den signierten Katalog gegenprüft
und den sie nach `daedyx/releases` legt. Die Datei ist öffentlich und gehört in Git. Sie fehlt noch, bis Björn den
Katalog-Schlüssel erzeugt hat (Anleitung: `docs/RELEASE-CI.md`). Der geheime Schlüssel kommt nie hierher.
