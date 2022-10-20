function showEmbed(e) {
  var parent = e.parentElement;
  parent.innerHTML = atob(parent.getAttribute('data-src'));
}

function showAllEmbeds(cl) {
  var list, attr;
  list = document.getElementsByClassName(`${cl} inline-embed`);
  for (var i = 0; i < list.length; i++) {
    attr = list[i].getAttribute('data-src');
    list[i].innerHTML = atob(attr);
  }
}

function expand(id) {
  var x = document.getElementById(id + "-more");
  var y = document.getElementById("triangle");
  if (x.style.display === "none") {
    x.style.display = "block";
    y.classList = "triangle down";
  } else {
    x.style.display = "none";
    y.classList = "triangle up";
  }
}

function show(section) {
  var x = document.getElementById(section + "-more");
  if (x.style.display === "none") {
    x.style.display = "block";
  } else {
    x.style.display = "none";
  }
  var y = document.getElementById(section + "-show");
  if (y.innerHTML === "[show]") {
    y.innerHTML = "[hide]";
  } else {
    y.innerHTML = "[show]";
  }
  document.getElementById(section).scrollIntoView();
}
