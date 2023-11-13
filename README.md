<h1 class="code-line" data-line-start=0 data-line-end=1 ><a id="TPWIKDPSTP01_0"></a>TP—WIK-DPS-TP01</h1>
<hr>
<h1 class="code-line" data-line-start=2 data-line-end=3 ><a id="TypeScript__2"></a>TypeScript :</h1>
<h1 class="code-line" data-line-start=3 data-line-end=4 ><a id="Installation__3"></a>Installation :</h1>
<p class="has-line-data" data-line-start="5" data-line-end="6">Utiliser les commandes suivante:</p>
<p class="has-line-data" data-line-start="7" data-line-end="14"><code>sudo apt install npm, git</code><br>
<code>Git-clone https://github.com/Vallsp/TP---WIK-DPS-TP01.git</code><br>
<code>cd TP---WIK-DPS-TP01</code><br>
<code>cd typescript</code><br>
<code>npm install</code><br>
<code>npx tsc --init --rootDir src --outDir build --esModuleInterop --resolveJsonModule --lib es6 --module commonjs --allowJs true --noImplicitAny true</code><br>
<code>node build/index.js</code></p>
<hr>
<h1 class="code-line" data-line-start=15 data-line-end=16 ><a id="Explication__15"></a>Explication :</h1>
<p class="has-line-data" data-line-start="17" data-line-end="18">Pour envoyer une requet la commande est : curl <a href="http://127.0.0.1/ping">http://127.0.0.1/ping</a></p>
<p class="has-line-data" data-line-start="19" data-line-end="20">Lorsque le serveur reçoit une requet GET sur /ping il renvoie les header en json  : “{“headers”:{“host”:“127.0.0.1:3000”,“user-agent”:“curl/8.0.1”,“accept”:”<em>/</em>“}}” par contre lorsqu’une autre requet GET différente de /ping est faite, le serveur ne renvoie rien.</p>
<p class="has-line-data" data-line-start="21" data-line-end="22">Il est possible de changer le port découte du serveur avec la variable d’environnement PING_LISTEN_PORT, il suffit de faire la commande suivante: “export PING_LISTEN_PORT = 8080”</p>
<p class="has-line-data" data-line-start="23" data-line-end="24">Si la variable n’est pas définie, le port par défaut est 3000, celui-ci peut aussi être modifier dans le index.ts “const defaultPort = 3000;”</p>
<hr>