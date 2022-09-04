let map;
let marker;

async function initMap() {
  map = new google.maps.Map(document.getElementById("map"), {
    center: { lat: 0, lng: 0 },
    zoom: 16,
  });

  initMarker('lancelot');
  initMarker('q')
}

async function initMarker(user) {
    const position = await fetch(`http://localhost:3001/location/${user}`).then(res => res.json());
    const gmapLatLng = new google.maps.LatLng(position.latitude, position.longitude),

    marker = new google.maps.Marker({ position: gmapLatLng, map: map });

    marker.setMap(marker);

    console.log("POSITION:", position);

    setInterval(() => updateMarkerPosition(map, user, marker), 1000)
}

async function updateMarkerPosition(map, user, marker) {
    const position = await fetch(`http://localhost:3001/location/${user}`).then(res => res.json());
    const gmapLatLng = new google.maps.LatLng(position.latitude, position.longitude);

    console.log("POSITION:", position);
    marker.setPosition(gmapLatLng);
    console.log("new position set");
}

window.initMap = initMap;
