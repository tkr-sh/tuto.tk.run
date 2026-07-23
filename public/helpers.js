var $ = e => e.startsWith('#') ? document.querySelector(e) : document.querySelectorAll(e);
HTMLElement.prototype.$ = function(e){return e.startsWith('#') ? this.querySelector(e) : this.querySelectorAll(e)};
var log=console.log
HTMLElement.prototype.on = function(t, l, o){this.addEventListener(t, l, o)};
HTMLElement.prototype.hasClass = function(c){return this.classList.contains(c)};
HTMLElement.prototype.addClass = function(c){this.classList.add(c)};
HTMLElement.prototype.rmClass = function(c){this.classList.remove(c)};
HTMLElement.prototype.add = function(e) {
    if (typeof e === "string") this.innerHTML += e
    else if (Array.isArray(e)) for (const c of e) this.append(c)
    else if (e instanceof HTMLElement) this.append(e)
    else throw "Invalid parameter type"
    return this;
};
HTMLElement.prototype.prep = function(e) {
    if (typeof e === "string") this.innerHTML = e + this.innerHTML
    else if (Array.isArray(e)) for (let i = e.length - 1; i >= 0; i--) this.prepend(array[i])
    else if (e instanceof HTMLElement) this.prepend(e)
    else throw "Invalid parameter type"
    return this;
};
HTMLElement.prototype.text = function(t=null) {
    if (t === null) {
        return this.textContent
    } else {
        this.textContent = t;
    }

};
HTMLElement.prototype.css = function(a, v=null){
    if (v == null) {
        return this.style[a]
    } else {
        console.log(this.style, a, v)
        this.style[a] = v;
    }
};
NodeList.prototype.each = function(fn){this.forEach(fn)}
var $new = (k, p, t, c) => {
    let e=document.createElement(k ?? 'div');
    for (let [a, v] of Object.entries(p)) e[a] = v;
    if (t !== null) e.text(t);
    if (c) for (let s of c) e.appendChild(s);
    return e;
}
var api = (method, url, data) => {
    const options = { method, headers: { 'Content-Type': 'application/json' } };
    if (data) options.body = JSON.stringify(data);
    return fetch(url, options).then(res => res.json());
};
var get = (url) => api('GET', url);
var post = (url, data) => api('POST', url, data);
var put = (url, data) => api('PUT', url, data);
var patch = (url, data) => api('PATCH', url, data);
var del = (url) => api('DELETE', url);

var _scrollPositions = {};
var _scrollSelectors = ['#main'];
var _saveScroll = () => {
    var entries = [];
    for (var sel of _scrollSelectors) {
        var el = document.querySelector(sel);
        if (el) entries.push({selector: sel, scrollTop: el.scrollTop});
    }
    if (entries.length) _scrollPositions[location.pathname] = entries;
};
var _restoreOrResetScroll = () => {
    var saved = _scrollPositions[location.pathname];
    if (saved) {
        for (var entry of saved) {
            var el = document.querySelector(entry.selector);
            if (el) {
                // This is to wait for putting the HTML fragment in the DOM
                setTimeout(() => el.scrollTo(0, entry.scrollTop), 10);
            }
        }
    } else {
        var mc = document.getElementById('main');
        if (mc) mc.scrollTo(0, 0);
    }
};

