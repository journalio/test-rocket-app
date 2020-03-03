<template>
    <div class="p-4">
        <h1 class="text-xl font-bold">Users lijst</h1>
        <ul>
            <li :key="user.email" v-for="user of users">
                {{ user.full_name }}
            </li>
        </ul>
        <button class="button">Nutteloze button</button>
    </div>
</template>

<script>
export default {
    data() {
        return {
            users: [],
        }
    },
    async mounted() {
        const result = await fetch('/api/user', {
            headers: {
                Accept: 'application/json',
            },
        })

        this.users = await result.json()
    },
}
</script>

<style scoped>
.button {
    @apply bg-blue-600 px-6 py-2 text-white shadow rounded uppercase;
}

.button:hover {
    @apply shadow-md bg-blue-700;
}
</style>
