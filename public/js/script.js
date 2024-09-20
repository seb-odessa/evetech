function zkbinfo() {
    // return "http://185.87.51.139:8080/api";
    return "http://localhost:8080/api";
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

async function requestBattleAsync(area, id) {
    const report = {};
    report.wins = await requestTotalAsync('wins', area, id);
    report.wins_sips = await requestReportAsync('wins', area, id, 'ships');
    report.wins_systems = await requestReportAsync('wins', area, id, 'systems');
    report.losses = await requestTotalAsync('losses', area, id);
    report.losses_sips = await requestReportAsync('losses', area, id, 'ships');
    report.losses_systems = await requestReportAsync('losses', area, id, 'systems');

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

