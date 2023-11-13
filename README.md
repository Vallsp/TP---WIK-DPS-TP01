# TP---WIK-DPS-TP01
--------------------------------------------
# TypeScript :
   # Installation :

Utiliser les commandes suivante:

 ```
 sudo apt install npm, git
 ```
```
Git-clone https://github.com/Vallsp/TP---WIK-DPS-TP01.git
```
```
cd TP---WIK-DPS-TP01
```
```
cd typescript
```
```
npm install
```
```
npx tsc --init --rootDir src --outDir build --esModuleInterop --resolveJsonModule --lib es6 --module commonjs --allowJs true --noImplicitAny true
```
```
node build/index.js
```
----------------------------------------
# Explication : 

Pour envoyer une requet la commande est : curl http://127.0.0.1/ping

Lorsque le serveur reçoit une requet GET sur /ping il renvoie les header en json  : "{"headers":{"host":"127.0.0.1:3000","user-agent":"curl/8.0.1","accept":"*/*"}}" par contre lorsqu'une autre requet GET différente de /ping est faite, le serveur ne renvoie rien.

Il est possible de changer le port découte du serveur avec la variable d'environnement PING_LISTEN_PORT, il suffit de faire la commande suivante: "export PING_LISTEN_PORT = 8080"

Si la variable n'est pas définie, le port par défaut est 3000, celui-ci peut aussi être modifier dans le index.ts "const defaultPort = 3000;"

-----------------


