<script lang="ts">
    import "./layout.css";

    const { children } = $props();
</script>

<main class="w-screen h-screen">
    {@render children()}
</main>
