
function zkbinfo() {
    return "http://185.87.51.139:8080/api";
    // return "http://192.168.0.100:8080/api";
}

function esi() {
    return "https://esi.evetech.net/latest";
}

function params() {
    return "?datasource=tranquility";
}

function makeRef(path, id, name) {
    const link = '/' + path + '/' + id + '/';
    return makeAnchor(link, name);
}

function makeAnchor(link, name) {
    const anchor = document.createElement('a');
    anchor.href = link;
    anchor.textContent = name;
    return anchor;
}

function makeImage(link, alt, width = 128, height = 128) {
    const img = document.createElement('img');
    img.src = link
    img.alt = alt
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

function getArray(id) {
    const namesDiv = document.getElementById(id);
    const namesText = namesDiv.textContent.trim().slice(1, -1);
    const namesArray = namesText.split(', ');
    return namesArray
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
    const cmd = suffix !== undefined ? `${suffix}/` : ""
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
    return data.map(([id, count]) => ({ id, count }));;
}

async function requestBattleAsync(area, id) {
    const report = new Object();
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


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

async function requestNamesAsync(ids) {
    const url = esi() + "/universe/names/?datasource=tranquility";
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


async function get(url = '') {
    const response = await fetch(url);
    return await response.json();
}

function sort_by_count(obj) {
    // let obj_map = new Map(Object.entries(obj).sort((a, b) => b[1] - a[1]).slice(0, count));
    let obj_map = new Map(Object.entries(obj).sort((a, b) => b[1] - a[1]));
    let map = new Map();
    obj_map.forEach((value, key) => { map.set(key, value) });
    return map;
}

function make_damage(damage) {
    return "<p>Total damage: " + damage + "</p>";
}

function make_items(msg, prefix, map, display = 6) {
    let html = [];
    html.push("<div>" + msg + ":&nbsp;");
    let idx = 0;
    map.forEach((count, id) => {
        html.push(`<div id="${prefix}_${id}" div style="display: inline">*</div> `);
        if (display == ++idx && idx != map.size) {
            html.push("<details><summary>More (" + (map.size - idx) + ") items...</summary>");
        } else if (idx == map.size) {
            html.push("</details>");
        }
    });
    html.push("</div>");
    return html.join("");
}

function update(category, prefix, names, map) {
    names.forEach((obj) => {
        const id = obj.id;
        const name = obj.name;
        const count = map.get(`${id}`);
        const href = `<a href="/gui/${category}/${name}/">${name} (${count})</a>`;
        const element = `${prefix}_${id}`;
        document.getElementById(element).innerHTML = href;
    });
}


function draw_prime_time(hourly) {
    const canvas = document.getElementById('prime_time').getContext('2d');
    const data = {
        datasets: [{
            label: 'killmails/hour',
            data: hourly,
            backgroundColor: 'rgba(255, 99, 132, 0.2)',
            borderColor: 'rgba(255, 99, 132, 1)',
            borderWidth: 1
        }]
    };

    const config = {
        type: 'bar',
        data: data,
        options: {
            responsive: false,
            scales: {
                y: {
                    beginAtZero: true
                }
            }
        }
    };

    const myChart = new Chart(canvas, config);
}