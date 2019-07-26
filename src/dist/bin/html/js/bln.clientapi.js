(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports) :
    typeof define === 'function' && define.amd ? define(['exports'], factory) :
    (factory((global.BlnClientApi = {})));
  }(this, (function (exports) { 'use strict';
    var host = "http://localhost:8283"
    function get(url, fn){
        var xhr = new XMLHttpRequest();        
        xhr.open('GET', url, true);
        xhr.onreadystatechange = function() {
        if (xhr.readyState == 4 && xhr.status == 200 || xhr.status == 304) { 
            fn.call(this, xhr.responseText);  
        }
        };
        xhr.send();
    }
    function post(url, data, fn){
        var xhr = new XMLHttpRequest();
        xhr.open("POST", url, true);
        xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");  
        xhr.onreadystatechange = function() {
            if (xhr.readyState == 4 && (xhr.status == 200 || xhr.status == 304)) {
                fn.call(this, xhr.responseText);
            }
        };
        xhr.send(data);
    }
    if (!self.get) {
        self.get = post;
        self.post = post;
    }
    exports.post = post;
    exports.get = get;
    Object.defineProperty(exports, '__esModule', { value: true });
})))

BlnClientApi.get("http://127.0.0.1:8283/api/v1/nodeinfo", function(data){
    console.log(JSON.parse(data));
})
// BlnClientApi.New()
// console.log(BlnClientApi.sum(1,2));
// 