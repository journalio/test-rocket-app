(async () => {
    const result = await fetch('/api/user');

    for (const user of await result.json()) {
        const p = document.createElement('p');
        p.innerText = `${user.email}: ${user.full_name}`;
        document.body.appendChild(p)
    }
})();