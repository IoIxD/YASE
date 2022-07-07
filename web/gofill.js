function ReadFile(name) {
	// I'm not even gonna try to use async, it would no doubt be faster but
	// it also starts a game of hot potato with a variable that can basically
	// only be used by other async functions, and clearly Golang can't handle it
	// either.
	var xhr = new XMLHttpRequest();
	var text = "";
	if (xhr != null) {
	  function load(e) {
	      text = xhr.responseText;
	  }
	  xhr.onerror = function(e) {
	      console.error(e);
	  }
	  try {
	    xhr.onload = load;
	  } catch(ex) {
	    xhr.readystatechange = load;
	  }
	  xhr.open('GET', name, false);
	  xhr.send(null);
	} else {
	  console.error("XMLHttpRequest not supported. Cannot continue properly.");
	  return;
	}
	return text;
}