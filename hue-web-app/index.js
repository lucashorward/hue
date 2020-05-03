const rust = import('hue_core_wasm');
const baseUri = `${config.baseuri}${config.token}`;
console.log(baseUri);
rust
    .then(m => {
        return m.get_lights(baseUri).then((data) => {
            console.log(data);
            const table = document.getElementById("table-body");
            for(var i in data) {
                let item = data[i];
                let row = table.insertRow();
                let id = row.insertCell(0);
                id.innerHTML = i;
                let name = row.insertCell(1);
                name.innerHTML = item.name;
                let on = row.insertCell(2);
                on.innerHTML = item.state.on;
                let brightness = row.insertCell(3);
                brightness.innerHTML = item.state.bri;
                let toggle = row.insertCell(4);
                toggle.innerHTML = `<button type="button" id="${i}"> Toggle</button>`
                let button = document.getElementById(i);
                button.addEventListener("click", function(e) {
                    m_copy = m;
                    id_copy = e.srcElement.id;
                    let light = m.LightState.new(!item.state.on);
                    m_copy.set_light_state(baseUri, id_copy, light).then(() => {
                        window.location.reload();
                    })
                });
              };
        })
    })
    .catch(console.error);