<script context="module">
	let prerender = false;
</script>

<script>
	import '../app.css';
</script>

<content>
	<nav>
		<ul>
			<li><a href="/"><img alt="uraeus" src="/onlydust.png" /></a></li>
			<li>uraeus</li>
		</ul>
	</nav>

	<slot />

	<footer>
		<p>built with ❤ by <a href="https://onlydust.xyz">onlydust.xyz</a></p>
	</footer>
</content>

<style>
	content {
		display: flex;
		flex-direction: column;
		flex-wrap: nowrap;
		justify-content: flex-start;
		align-items: stretch;
		align-content: stretch;
		height: 100vh;
	}

	nav {
		display: flex;
		flex-direction: row;
		justify-content: center;
		margin: 0px;
		background-color: black;
	}

	nav ul {
		padding: 0px;
		margin: 0px;
		flex-grow: 100;
		max-width: 1024px;
		list-style-type: none;
		display: flex;
		flex-direction: row;
		justify-content: left;
		align-items: center;
		padding-top: 4px;
		padding-bottom: 4px;
		background-color: black;
	}

	nav ul img {
		width: 64px;
		padding-left: 10px;
	}

	nav ul li {
		color: #fff;
		font-size: 1.5rem;
		padding: 0px 4px;
	}

	footer p a {
		color: hotpink;
	}

	footer p a:link {
		color: hotpink;
	}

	/* visited link */
	footer p a:visited {
		color: hotpink;
	}

	/* mouse over link */
	footer p a:hover {
		color: orangered;
	}

	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding-top: 20px;
		padding-bottom: 20px;
		background-color: black;
	}

	footer p {
		color: #fff;
		font-size: 1.2rem;
	}

	footer a {
		font-weight: bold;
	}

	@media (min-width: 480px) {
		footer {
			padding-top: 20px;
			padding-bottom: 0px;
		}
	}
</style>
