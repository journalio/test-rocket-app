const BASE_PATH = '/api/user'

export default class UserService {
    async create(user) {
        const result = await fetch(BASE_PATH, {
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            method: 'POST',
            body: JSON.stringify(user),
        })

        return result.json()
    }

    async all() {
        const result = await fetch(BASE_PATH, {
            headers: {
                Accept: 'application/json',
            },
        })

        return result.json()
    }

    async findById(id) {
        const result = await fetch(`${BASE_PATH}/${id}`, {
            headers: {
                Accept: 'application/json',
            },
        })
        return result.json()
    }
}
