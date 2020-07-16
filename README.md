<h1><code>wthr</code></h1>
<p>A Cli app designed to give you the weather of any location</p>
<p>this is the backend. The  Frontend can be found <a href="https://github.com/ShaneM123/wthr">here</a></p>

<h2> to run </h2>
<p> in its current prototype state it requires rust installed and an api key for openweather.org (this will be fixed shortly)</p>

<p>to test this backend its current state firstly add an openweather.org apikey to the .env file:</p>
<code>APP_ID= "api key goes here"
</code>
<p>then cargo run in the project folder (requires rust installed):</p>
<code>cargo run</code>

<p> then enter the following url in your browser or you can use postman/curl if you wish:</p>
<code>http://127.0.0.1:8445/todayweatherbycity/Galway/IE</code>
<p></p>
<p>the last part of the url is the country code. </p>