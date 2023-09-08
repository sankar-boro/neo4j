const { users, ifs } = require("./data");

function run() {
    const isRun = false;

    const tables = {
        marriedTable: [],
        canDrinkTable: [],
        isRichTable: [],
    }

    ifs.forEach((thisif) => {
        users.forEach((user) => {
            const fn_ = thisif['fn'];
            const res = fn_(user[thisif.name], thisif.value);
            if (res) {
                tables[thisif.linkedTable].push(user);
            }
        })
    })

    console.log(tables)
}

run()