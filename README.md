<h1 class="code-line" data-line-start=0 data-line-end=1 ><a id="TPWIKDPSTP01_0"></a>TP—WIK-DPS-TP01</h1>
<p class="has-line-data" data-line-start="2" data-line-end="3">Installation :</p>
<p class="has-line-data" data-line-start="4" data-line-end="12">sudo apt install npm, git<br>
Git-clone <a href="https://github.com/Vallsp/TP">https://github.com/Vallsp/TP</a>—WIK-DPS-TP01.git<br>
cd TP—WIK-DPS-TP01<br>
cd typescript<br>
npm init -y<br>
npm install<br>
npx tsc --init --rootDir src --outDir build --esModuleInterop --resolveJsonModule --lib es6 --module commonjs --allowJs true --noImplicitAny true<br>
node build/index.js</p>
<p class="has-line-data" data-line-start="13" data-line-end="14">Explication :</p>
<p class="has-line-data" data-line-start="15" data-line-end="16">Pour envoyer une requet la commande est : curl <a href="http://127.0.0.1/ping">http://127.0.0.1/ping</a></p>
<p class="has-line-data" data-line-start="17" data-line-end="18">Lorsque le serveur reçoit une requet GET sur /ping il renvoie les header en json  : “{“headers”:{“host”:“127.0.0.1:3000”,“user-agent”:“curl/8.0.1”,“accept”:”<em>/</em>“}}” par contre lorsqu’une autre requet GET différente de /ping est faite, le serveur ne renvoie rien.</p>
<p class="has-line-data" data-line-start="19" data-line-end="20">Il est possible de changer le port découte du serveur avec la variable d’environnement PING_LISTEN_PORT, il suffit de faire la commande suivante: “export PING_LISTEN_PORT = 8080”</p>
<p class="has-line-data" data-line-start="21" data-line-end="22">Si la variable n’est pas définie, le port par défaut est 3000, celui-ci peut aussi être modifier dans le index.ts “const defaultPort = 3000;”</p>