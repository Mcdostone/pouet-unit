import './style.css'
import init, { greet } from 'pouet_jane';



await init()


const input = document.querySelector('#app input') as HTMLInputElement;
const result = document.querySelector('#app #result') as HTMLOutputElement;
function convert(payload: string) {
  // @ts-expect-error la flemme
  result.value = greet(payload)
}

input?.addEventListener('input', e => {
  e.preventDefault();
  // @ts-expect-error la flemme
  convert(e.target.value)
});
document.querySelector('#app form')?.addEventListener('submit', e => {
  e.preventDefault();
  convert(input?.value)
})


convert(input?.value);