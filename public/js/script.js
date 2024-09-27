function zkbinfo() {
    return "http://185.87.51.139:8080/api";
    // return "http://localhost:8080/api";
}

function esi() {
    return "https://esi.evetech.net/latest";
}

function params() {
    return "?datasource=tranquility";
}

function makeRef(path, id, name) {
    return makeAnchor('/' + path + '/' + id + '/', name);
}

function makeAnchor(link, name) {
    const anchor = document.createElement('a');
    anchor.href = link;
    anchor.textContent = name;
    return anchor;
}

function makeImage(link, alt, width = 128, height = 128) {
    const img = document.createElement('img');
    img.src = link;
    img.alt = alt;
    img.width = width;
    img.height = height;
    return img;
}

function makeParagraph(content) {
    const paragraph = document.createElement('p');
    paragraph.textContent = content;
    return paragraph;
}

function insertInto(id, element) {
    const parent = document.getElementById(id);
    parent.innerHTML = '';
    parent.appendChild(element);
}

function methodGet(method) {
    return {
        method: 'GET',
        mode: 'cors',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'follow',
        referrerPolicy: 'no-referrer',
    };
}

async function requestIdsAsync(names) {
    const url = esi() + "/universe/ids/" + params();
    const response = await fetch(url, {
        method: 'POST',
        mode: 'cors',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'follow',
        referrerPolicy: 'no-referrer',
        body: JSON.stringify(names)
    });
    return await response.json();
}

async function getObjectAsync(subject, id, suffix) {
    const cmd = suffix !== undefined ? `${suffix}/` : "";
    const url = esi() + '/' + subject + '/' + id + '/' + cmd + params();
    const cached = localStorage.getItem(url);
    if (cached) {
        return JSON.parse(cached);
    }
    const response = await fetch(url, methodGet());
    if (!response.ok) {
        console.log(`${response.status}: ${response.statusText}`);
    }
    const data = await response.json();
    localStorage.setItem(url, JSON.stringify(data));
    return data;
}

async function requestTotalAsync(prefix, area, id) {
    const url = zkbinfo() + '/' + prefix + '/' + area + '/' + id;
    const response = await fetch(url, methodGet());
    if (!response.ok) {
        console.log(`${response.status}: ${response.statusText}`);
    }
    const data = await response.json();
    return {
        count: data[0],
        damage: data[1]
    };
}

async function requestReportAsync(prefix, area, id, cmd) {
    const url = zkbinfo() + '/' + prefix + '/' + area + '/' + id + '/' + cmd;
    const response = await fetch(url, methodGet());
    if (!response.ok) {
        console.log(`${response.status}: ${response.statusText}`);
    }
    const data = await response.json();
    return data.map(([id, count]) => ({ id, count }));
}

async function requestFriendlyAsync(object, subject, id) {
    const url = zkbinfo() + '/friendly/' + object + '/for/' + subject + '/' + id;
    const response = await fetch(url, methodGet());
    if (!response.ok) {
        console.log(`${response.status}: ${response.statusText}`);
    }
    const data = await response.json();
    return data.map(([id, count]) => ({ id, count }));
}

async function requestEnemyAsync(object, subject, id) {
    const url = zkbinfo() + '/enemy/' + object + '/for/' + subject + '/' + id;
    const response = await fetch(url, methodGet());
    if (!response.ok) {
        console.log(`${response.status}: ${response.statusText}`);
    }
    const data = await response.json();
    return data.map(([id, count]) => ({ id, count }));
}

async function requestNamesAsync(ids) {
    const url = esi() + "/universe/names/" + params();
    const response = await fetch(url, {
        method: 'POST',
        mode: 'cors',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'follow',
        referrerPolicy: 'no-referrer',
        body: JSON.stringify(ids)
    });
    return await response.json();
}

async function getNames(ids) {
    const unique = [...new Set(ids)];
    const names = await requestNamesAsync(unique);
    return names.reduce((acc, item) => {
        acc[item.id] = item.name;
        return acc;
    }, {});
}

function buildRecords(items, names){
    return items.sort((a, b) => b.count - a.count).map(item => ({
        id: item.id,
        name: names[item.id] || 'Unknown',
        count: item.count
    }));
}

function createTableHead(headers) {
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    var column = 0;
    headers.forEach(headerText => {
        const th = document.createElement('th');
        th.textContent = headerText;
        th.className = "column_header_" + (++column);
        headerRow.appendChild(th);
    });
    thead.appendChild(headerRow);
    return thead;
}

function createTable(refference, data, rows = 10) {
    const table = document.createElement('table');
    table.appendChild(createTableHead(['Имя', 'Счёт']));

    const tbody = document.createElement('tbody');
    var rowCount = 0;
    data.forEach(item => {
        ++rowCount;
        const row = document.createElement('tr');
        if (rowCount > rows) {
            row.className = "hidden";
        }

        const cellName = document.createElement('td');
        const link = refference.replace('$(id)', item.id);
        cellName.appendChild(makeAnchor(link, item.name));
        cellName.className = 'allign_left';
        row.appendChild(cellName);

        const cellWins = document.createElement('td');
        cellWins.textContent = item.count;
        cellWins.className = 'allign_right';
        row.appendChild(cellWins);

        tbody.appendChild(row);
    });

    table.appendChild(tbody);
    return table;
}

async function requestShortReportAsync(area, id) {
    const report = {};
    report.wins = await requestTotalAsync('wins', area, id);
    report.losses = await requestTotalAsync('losses', area, id);

    const total_count = report.wins.count + report.losses.count;
    if (total_count > 0) {
        report.wins.count_percent = (100 * report.wins.count / total_count).toFixed(2);
        report.losses.count_percent = (100 * report.losses.count / total_count).toFixed(2);
    }

    const total_damage = report.wins.damage + report.losses.damage;
    if (total_damage > 0) {
        report.wins.damage_percent = (100 * report.wins.damage / total_damage).toFixed(2);
        report.losses.damage_percent = (100 * report.losses.damage / total_damage).toFixed(2);
    }

    return report;
}

async function requestShipReportAsync(area, id) {
    const report = {};
    const win_ships = await requestReportAsync('wins', area, id, 'ships');
    const lost_ships = await requestReportAsync('losses', area, id, 'ships');
    const ship_names = await getNames(win_ships.concat(lost_ships).map(item => item.id));
    report.win_ships = buildRecords(win_ships, ship_names);
    report.lost_ships = buildRecords(lost_ships, ship_names);
    return report;
}

async function requestSystemReportAsync(area, id) {
    const report = {};
    const win_systems = await requestReportAsync('wins', area, id, 'systems');
    const lost_systems = await requestReportAsync('losses', area, id, 'systems');
    const system_names = await getNames(win_systems.concat(lost_systems).map(item => item.id));
    report.win_systems = buildRecords(win_systems, system_names);
    report.lost_systems = buildRecords(lost_systems, system_names);
    return report;
}

async function requestFriendsAndEnemyAsync(object, subject, id) {
    const report = {};

    const friends = await requestFriendlyAsync(object, subject, id);
    const enemy = await requestEnemyAsync(object, subject, id);
    const names = await getNames(friends.concat(enemy).map(item => item.id));
    report.friends = buildRecords(friends, names);
    report.enemy = buildRecords(enemy, names);

    // const friendly_corps = await requestFriendlyAsync("corporation", "character", id);
    // const enemy_corps = await requestEnemyAsync("corporation", "character", id);
    // const corp_names = await getNames(friendly_corps.concat(enemy_corps).map(item => item.id));
    // report.friendly_corps = buildRecords(friendly_corps, corp_names);
    // report.enemy_corps = buildRecords(enemy_corps, corp_names);

    // const friendly_allis = await requestFriendlyAsync("alliance", "character", id);
    // const enemy_allis = await requestEnemyAsync("alliance", "character", id);
    // const alli_names = await getNames(friendly_allis.concat(enemy_allis).map(item => item.id));
    // report.friendly_allis = buildRecords(friendly_allis, alli_names);
    // report.enemy_allis = buildRecords(enemy_allis, alli_names);

    return report;
}