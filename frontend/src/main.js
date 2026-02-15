import * as wasm from "pouet_jane";

async function main() {
  const { convert_units, Unit } = wasm;

  const units = [
    { value: Unit.Liter, label: 'Liter' },
    { value: Unit.Gallon, label: 'Gallon' },
  ];


  document.querySelector('#app').innerHTML = `
    <div class="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-blue-100 via-purple-100 to-pink-100 p-4">
      <div class="w-full max-w-lg bg-white/80 backdrop-blur-md rounded-3xl shadow-2xl p-8 flex flex-col gap-8 border border-purple-200">
        <h1 class="text-4xl font-extrabold text-center text-purple-700 tracking-tight mb-2 drop-shadow">Unit Converter</h1>
        <p class="text-center text-gray-500 mb-4">Convert between gallons and liters instantly!</p>
        <div class="flex flex-col md:flex-row gap-6 items-center justify-center">
          <div class="flex-1 flex flex-col gap-2">
            <label class="text-sm font-semibold text-gray-700" for="value">Value</label>
            <input id="value" type="number" class="w-full border-2 border-purple-200 focus:border-purple-400 rounded-xl px-4 py-3 text-lg shadow-sm focus:outline-none transition" value="1" min="0" step="any" />
          </div>
          <div class="flex-1 flex flex-col gap-2">
            <label class="text-sm font-semibold text-gray-700" for="from">From</label>
            <select id="from" class="w-full border-2 border-purple-200 focus:border-purple-400 rounded-xl px-4 py-3 text-lg shadow-sm focus:outline-none transition">
              ${units.map(u => `<option value="${u.value}">${u.label}</option>`).join('')}
            </select>
          </div>
          <div class="flex-1 flex flex-col gap-2">
            <label class="text-sm font-semibold text-gray-700" for="to">To</label>
            <select id="to" class="w-full border-2 border-purple-200 focus:border-purple-400 rounded-xl px-4 py-3 text-lg shadow-sm focus:outline-none transition">
              ${units.map(u => `<option value="${u.value}">${u.label}</option>`).join('')}
            </select>
          </div>
        </div>
        <div class="flex flex-col items-center mt-2">
          <span class="text-lg text-gray-600 font-medium">Result</span>
          <span id="result" class="mt-2 text-3xl font-bold text-purple-700 drop-shadow">-</span>
        </div>
        <div class="flex justify-center mt-4">
          <span class="text-xs text-gray-400">Powered by <span class="font-semibold text-purple-500">pouet_jane</span> WASM</span>
        </div>
      </div>
    </div>
  `;

  const valueInput = document.getElementById('value');
  const fromSelect = document.getElementById('from');
  const toSelect = document.getElementById('to');
  const resultSpan = document.getElementById('result');

  function updateResult() {
    const value = parseFloat(valueInput.value);
    const from = parseInt(fromSelect.value, 10);
    const to = parseInt(toSelect.value, 10);
    if (isNaN(value)) {
      resultSpan.textContent = '-';
      return;
    }
    const result = convert_units(value, from, to);
    resultSpan.textContent = isNaN(result) ? 'N/A' : result;
  }

  valueInput.addEventListener('input', updateResult);
  fromSelect.addEventListener('change', updateResult);
  toSelect.addEventListener('change', updateResult);

  updateResult();
}

main();
