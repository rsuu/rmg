import init, { Img, ZipExtract } from "/wasm/yiiy.js";

// REFS: https://stackoverflow.com/a/1216743
// Anonymous function to not pollute global namespace
(function () {
  const DEBUG = true;
  const LEVEL = ["debug", "log", "info", "warn"];

  if (!DEBUG) {
    if (!window.console) {
      window.console = {} as Console;
    }

    for (const i in LEVEL) {
      console[LEVEL[i] as keyof Console] = function () {};
    }
  }
})();

enum Dire {
  Up,
  Down,
  Stop,
}

const WASM = "/wasm/yiiy.wasm";
const MAX_LEN = 4;
const SLEEP = 150;

let head = 0;
let tail = 0;
let dire = Dire.Down;

let currentScroll = 0;
let tailScrollTop = 0;
let tail_prec = 0;

let zip_input = document.querySelector("#zip_input") as HTMLInputElement;
let img_list = document.querySelector("#img_list") as HTMLDivElement;

let btn_get = document.querySelector("#btn_get") as HTMLButtonElement;
let btn_get_url = document.querySelector("#btn_get_url") as HTMLInputElement;

let tail_page = document.querySelector("#tail") as HTMLParagraphElement;

async function main() {
  await init(WASM);

  console.debug("init: wasm");

  let app = new App();
  app.main();

  // let div = document.getElementById("__js");
}
main();

export class App {
  public version: string;

  constructor() {
    this.version = "v0.0.0";
  }

  public async main() {
    zip_input.addEventListener("change", event_zip_input);
    btn_get.addEventListener("click", event_btn_get);
  }
}

async function event_zip_input(event: Event) {
  let file = (event.target as HTMLInputElement).files![0];
  let reader = new FileReader();

  reader.onload = function (event) {
    // document.querySelectorAll("#img_list img").forEach((e) => { e.remove(); });

    // hide
    (document.querySelector("#top") as HTMLDivElement).style.display = "none";

    let arrayBuffer = (event.target as FileReader).result as ArrayBuffer;
    let bytes = new Uint8Array(arrayBuffer);

    let zip = new ZipExtract(bytes);
    let filelist = zip.get_list();
    let len = filelist.length;

    console.debug(`zip len: ${len}`);

    check_dire();
    loop(zip, filelist);
  };
  reader.readAsArrayBuffer(file);
}

function check_dire() {
  window.addEventListener("scroll", function () {
    currentScroll = window.pageYOffset || document.documentElement.scrollTop;

    if (currentScroll > tailScrollTop) {
      dire = Dire.Down;
    } else if (currentScroll < tailScrollTop) {
      dire = Dire.Up;
    }

    // reset tailScrollTop at top.
    if (currentScroll <= 0) {
      tailScrollTop = 0;
    } else {
      tailScrollTop = currentScroll;
    }
  });
}

function loop(zip: ZipExtract, filelist: Img[]) {
  setTimeout(function () {
    if (dire == Dire.Down) {
      scroll_down(zip, filelist);
    } else if (dire == Dire.Up) {
      scroll_up(zip, filelist);
      dire = Dire.Stop;
    }

    console.debug(dire);

    loop(zip, filelist);
  }, SLEEP);
}

function scroll_down(zip: ZipExtract, filelist: Img[]) {
  for (const element of filelist) {
    let win_h = document.body.scrollHeight - window.innerHeight;
    let prec = (window.pageYOffset / win_h) * 100;
    let page_number = element.page_number();

    console.debug(`#DOWN head: ${head}, tail: ${tail}, prec: ${prec}`);

    if (tail == filelist.length) {
      return;
    }

    if (tail <= head + MAX_LEN && page_number == tail + 1) {
      console.debug(page_number, tail);

      let name = element.get_name();
      let idx = element.idx();
      let data = zip.get_slice(idx);
      let len = data.length;

      let blob = new Blob([data.buffer]);
      let url = URL.createObjectURL(blob);

      let img = new Image();
      img.src = url;
      img.setAttribute("page_number", page_number.toString());
      img.addEventListener("click", function () {
        console.debug(page_number);
      });

      img_list.appendChild(img);
      tail += 1;

      console.debug(`name: ${name}\ndata_len: ${len}`);
      break;
    }

    // free
    if (tail >= head + MAX_LEN && prec > 50) {
      let imgs = document.querySelectorAll<HTMLImageElement>("#img_list img");

      for (const img of Array.from(imgs)) {
        let n = parseInt(img.getAttribute("page_number")!);
        if (n == head + 1) {
          console.debug(
            `n: ${n}, free-head: ${head}, tail: ${tail}, prec: ${prec}`,
          );

          img.remove();
          head += 1;

          break;
        }
      }
    }

    let count = document.querySelectorAll("#img_list img").length;

    tail_page.textContent! = count.toString();

    tail_prec = prec;
  }
}

// FIXME: always up in scroll fast
function scroll_up(zip: ZipExtract, filelist: Img[]) {
  for (const element of filelist) {
    let win_h = document.body.scrollHeight - window.innerHeight;
    let prec = (window.pageYOffset / win_h) * 100;
    let page_number = element.page_number();

    console.debug(`#UP head: ${head}, tail: ${tail}, prec: ${prec}`);

    if (head == 0) {
      return;
    }

    if (tail <= head + MAX_LEN && page_number == head) {
      let name = element.get_name();
      let idx = element.idx();
      let data = zip.get_slice(idx);
      let len = data.length;

      let blob = new Blob([data.buffer]);
      let url = URL.createObjectURL(blob);

      let img = new Image();
      img.src = url;
      img.setAttribute("page_number", page_number.toString());
      img.addEventListener("click", function () {
        console.debug(page_number);
      });

      img_list.insertBefore(img, img_list.firstChild);
      head -= 1;

      console.debug(`name: ${name}\ndata_len: ${len}`);

      break;
    }

    if (tail >= head + MAX_LEN && prec < 50) {
      let imgs = document.querySelectorAll<HTMLImageElement>("#img_list img");

      for (const img of Array.from(imgs)) {
        let n = parseInt(img.getAttribute("page_number")!);
        if (n == tail) {
          console.debug(
            `free_up: n: ${n}, head: ${head}, tail: ${tail}, prec: ${prec}`,
          );

          img.remove();
          tail -= 1;

          break;
        }
      }
    }

    let count = document.querySelectorAll("#img_list img").length;

    tail_page.textContent! = count.toString();

    tail_prec = prec;
  }
}

async function event_btn_get() {
  console.debug(`bind_get_file`);

  await get_file().catch(function (e) {
    console.debug(e);
  });
}

async function get_file() {
  let headers = new Headers();

  headers.append("Content-Type", "application/json");
  headers.append("Accept", "application/json");

  //    headers.append("Access-Control-Allow-Origin", "*");
  //    headers.append(
  //      "Access-Control-Allow-Methods",
  //      "POST, GET, OPTIONS, DELETE",
  //    );
  //    headers.append("Access-Control-Max-Age", "3600");
  //    headers.append("Access-Control-Allow-Headers", "x-requested-with");

  ////指定允许其他域名访问
  //'Access-Control-Allow-Origin:http://172.20.0.206'//一般用法（*，指定域，动态设置），3是因为*不允许携带认证头和cookies
  ////是否允许后续请求携带认证信息（cookies）,该值只能是true,否则不返回
  //'Access-Control-Allow-Credentials:true'
  ////预检结果缓存时间,也就是上面说到的缓存啦
  //'Access-Control-Max-Age: 1800'
  ////允许的请求类型
  //'Access-Control-Allow-Methods:GET,POST,PUT,POST'
  ////允许的请求头字段
  //'Access-Control-Allow-Headers:x-requested-with,content-type'

  let response = await fetch(btn_get_url.value, {
    headers: headers,
  });

  let blob = await response.blob();
  let tmp = await blob.arrayBuffer();
  let bytes = new Uint8Array(tmp);

  console.debug(bytes.length);

  // Create a temporary link element
  // let link = document.createElement("a");
  // link.href = URL.createObjectURL(blob);
  // Set the default file name here
  // link.download = "downloadedFile";
  // link.click();

  // Clean up
  // URL.revokeObjectURL(link.href);
}
