<script>
	import Header from './Header.svelte';
	import "../app.css";
	import gpu_icon from '$lib/images/output-onlinegiftools.gif';
</script>

<div class="app">
	<section class="bg-black">
		<Header />
	</section>

	<main class="p-7">
		<div class="lg:flex items-center">
			<div class="flex flex-col">
				<div class="text-8xl font-extrabold">GPU compute that scales WITH YOU</div>
				<div class="pt-9 text-2xl max-w-[480px]">GPUse is a gateway to on-demand, partitioned GPU access for developers. With GPUse, you only pay for the precise GPU compute power you require to train AI models or run inference when you need it.</div>
			</div>
				<img src={gpu_icon} alt="GPUse" />
		</div>
	</main>
</div>

