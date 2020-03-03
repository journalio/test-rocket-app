<template>
    <div class="p-4">
        <h1 class="text-xl font-bold">Users lijst</h1>
        <ul>
            <li :key="user.email" v-for="user of users">{{ user.full_name }}</li>
        </ul>
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

</style>