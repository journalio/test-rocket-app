<template>
    <div class="p-4">
        <h1 class="text-xl font-bold">Users list</h1>
        <ul>
            <li :key="user.email" v-for="user of users">
                {{ user.full_name }}
            </li>
        </ul>

        <form @submit.prevent="submit()" class="border-t py-4 my-4">
            <h2 class="text-lg font-bold">Create user</h2>
            <label class="label">
                Full name
                <input
                    type="text"
                    name="full_name"
                    v-model="newUser.full_name"
                    autocomplete="name"
                    class="input"
                />
            </label>
            <label class="label">
                Password
                <input
                    type="password"
                    name="password_hash"
                    v-model="newUser.password_hash"
                    autocomplete="name"
                    class="input"
                />
            </label>
            <label class="label">
                Email
                <input
                    type="email"
                    name="email"
                    v-model="newUser.email"
                    autocomplete="name"
                    class="input"
                />
            </label>

            <label
                ><input
                    type="submit"
                    class="button"
                    :disabled="loading"
                    value="Create user"
            /></label>
        </form>
    </div>
</template>

<script>
import UserService from './services/UserService'

const userService = new UserService()

export default {
    methods: {
        async submit() {
            await userService.create(this.newUser)
            this.resetForm()

            await this.fetchUsers()
        },
        async fetchUsers() {
            this.users = await userService.all()
            this.loading = false
        },
        resetForm() {
            this.newUser = {
                email: '',
                password_hash: '',
                full_name: '',
            }
        },
    },
    data() {
        return {
            loading: true,
            users: [],
            newUser: { email: '', password_hash: '', full_name: '' },
        }
    },
    mounted() {
        this.fetchUsers()
    },
}
</script>

<style scoped>
.button {
    @apply bg-blue-600 px-4 py-1 text-white shadow rounded uppercase;
}

.button:hover {
    @apply shadow-md bg-blue-700;
}

.label {
    @apply block my-4;
}

.input {
    @apply border rounded px-2 py-1 text-black;
}
</style>
