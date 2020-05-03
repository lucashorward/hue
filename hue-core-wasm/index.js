const rust = import('./pkg');
const config = import('./config');
const baseUri = `${config.baseuri}${config.token}`;
rust
    .then(m => {
        let light = m.LightState.new(false, undefined, undefined, undefined);
        m.set_light_state(baseUri, "1", light)
        return m.get_lights(baseUri).then((data) => {
            console.log(data);
        })
    })
    .catch(console.error);