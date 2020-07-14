<h1><code>wthr</code></h1>
<p>A Cli app designed to give you the weather of any location</p>
<p>this is the backend. The  Frontend can be found <a href="https://github.com/ShaneM123/wthr">here</a></p>

<p>work in progress, currently working on handling the api response. Want it to look something like this:</p>
<p>$: wthr Berlin DE today </p>
<p>$: Sunny, 25 Degrees Celsius  </p>

<h2> to run </h2>
<p> in its current prototype state it requires rust installed and an api key for openweather.org (this will be fixed shortly)</p>

<p>to test this backend its current state firstly add an openweather.org apikey to the .env file:</p>
<code>APP_ID= "api key goes here"
</code>
<p>then cargo run in the project folder (requires rust installed):</p>
<code>cargo run</code>

<p> then enter the following url in your browser or you can use postman/curl if you wish(no https currently):</p>
<code>127.0.0.1:8445/todayweatherbycity/2964180</code>

<p>the last part of the url is the city code. valid city id's can be found in the citylist.json file and api keys can be obtained by signing up to openweather.org</p>