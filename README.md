<h1 class="code-line" data-line-start=0 data-line-end=1 ><a id="TPWIKDPSTP01_0"></a>TP—WIK-DPS-TP01</h1>
<hr>
<h1 class="code-line" data-line-start=2 data-line-end=3 ><a id="TypeScript__2"></a>TypeScript :</h1>
<h1 class="code-line" data-line-start=3 data-line-end=4 ><a id="Installation__3"></a>Installation :</h1>
<p class="has-line-data" data-line-start="5" data-line-end="6">Utiliser les commandes suivante:</p>
<ul>
<li class="has-line-data" data-line-start="7" data-line-end="9">
<p class="has-line-data" data-line-start="7" data-line-end="8">sudo apt install npm, git</p>
</li>
<li class="has-line-data" data-line-start="9" data-line-end="10">
<p class="has-line-data" data-line-start="9" data-line-end="10">Git-clone <a href="https://github.com/Vallsp/TP">https://github.com/Vallsp/TP</a>—WIK-DPS-TP01.git</p>
</li>
<li class="has-line-data" data-line-start="10" data-line-end="11">
<p class="has-line-data" data-line-start="10" data-line-end="11">cd TP—WIK-DPS-TP01</p>
</li>
<li class="has-line-data" data-line-start="11" data-line-end="12">
<p class="has-line-data" data-line-start="11" data-line-end="12">cd my_project</p>
</li>
<li class="has-line-data" data-line-start="12" data-line-end="13">
<p class="has-line-data" data-line-start="12" data-line-end="13">npm install</p>
</li>
<li class="has-line-data" data-line-start="13" data-line-end="14">
<p class="has-line-data" data-line-start="13" data-line-end="14">npx tsc --init --rootDir src --outDir build --esModuleInterop --resolveJsonModule --lib es6 --module commonjs --allowJs true --noImplicitAny true</p>
</li>
<li class="has-line-data" data-line-start="14" data-line-end="15">
<p class="has-line-data" data-line-start="14" data-line-end="15">node build/index.js</p>
</li>
</ul>
<hr>
<h1 class="code-line" data-line-start=16 data-line-end=17 ><a id="Explication__16"></a>Explication :</h1>
<p class="has-line-data" data-line-start="18" data-line-end="19">Pour envoyer une requet la commande est : curl <a href="http://127.0.0.1/ping">http://127.0.0.1/ping</a></p>
<p class="has-line-data" data-line-start="20" data-line-end="21">Lorsque le serveur reçoit une requet GET sur /ping il renvoie les header en json  : “{“headers”:{“host”:“127.0.0.1:3000”,“user-agent”:“curl/8.0.1”,“accept”:”<em>/</em>“}}” par contre lorsqu’une autre requet GET différente de /ping est faite, le serveur ne renvoie rien.</p>
<p class="has-line-data" data-line-start="22" data-line-end="23">Il est possible de changer le port découte du serveur avec la variable d’environnement PING_LISTEN_PORT, il suffit de faire la commande suivante: “export PING_LISTEN_PORT = 8080”</p>
<p class="has-line-data" data-line-start="24" data-line-end="25">Si la variable n’est pas définie, le port par défaut est 3000, celui-ci peut aussi être modifier dans le index.ts “const defaultPort = 3000;”</p>
<hr>