<!-- <a href="/">to home</a> -->
<h1>daily</h1>

<nav class="nav-bar">
<ul>
    <li><a href="#">abc</a></li>
    <li><a href="#">cba</a></li>
</ul>
</nav>

<style>
nav {
    border-bottom: 1px solid black;
}

.nav-bar ul {
    display: inline-block;
    list-style-type: none;
    padding-left: 0;
}

.nav-bar a::after {
    display: inline-block;
    color: black;
    content: ">";
}

</style>