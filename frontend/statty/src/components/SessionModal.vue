<template>
  <TransitionRoot as="template" :show="props.open">
    <Dialog as="div" class="relative z-10" @close="$emit('toggleModal')">
      <div class="fixed inset-0" />

      <div class="fixed inset-0 overflow-hidden">
        <div class="absolute inset-0 overflow-hidden">
          <div class="pointer-events-none fixed inset-y-0 right-0 flex max-w-full pl-10 sm:pl-16">
            <TransitionChild as="template" enter="transform transition ease-in-out duration-500 sm:duration-700" enter-from="translate-x-full" enter-to="translate-x-0" leave="transform transition ease-in-out duration-500 sm:duration-700" leave-from="translate-x-0" leave-to="translate-x-full">
              <DialogPanel class="pointer-events-auto w-screen max-w-md">
                <form class="flex h-full flex-col divide-y divide-gray-200 bg-white shadow-xl">
                  <div class="h-0 flex-1 overflow-y-auto">
                    <div class="bg-indigo-700 px-4 py-6 sm:px-6">
                      <div class="flex items-center justify-between">
                        <DialogTitle class="text-base font-semibold leading-6 text-white">Add a session</DialogTitle>
                        <div class="ml-3 flex h-7 items-center">
                          <button type="button" class="relative rounded-md bg-indigo-700 text-indigo-200 hover:text-white focus:outline-none focus:ring-2 focus:ring-white" @click="$emit('toggleModal')">
                            <span class="absolute -inset-2.5" />
                            <span class="sr-only">Close panel</span>
                            <XMarkIcon class="h-6 w-6" aria-hidden="true" />
                          </button>
                        </div>
                      </div>
                      <div class="mt-1">
                        <p class="text-sm text-indigo-300">Enter the information of your charging session below.</p>
                      </div>
                    </div>
                    <div class="flex flex-1 flex-col justify-between">
                      <div class="divide-y divide-gray-200 px-4 sm:px-6">
                        <div class="space-y-6 pb-5 pt-6">
                          <div>
                            <label for="project-name" class="block text-sm font-medium leading-6 text-gray-900">Date</label>
                            <div class="mt-2">
                              <VueDatePicker v-model="session.date" :format="format" :enable-time-picker="false"></VueDatePicker>
                            </div>
                          </div>
                          <div>
                            <label for="price" class="block text-sm font-medium leading-6 text-gray-900">Energy Added</label>
                            <div class="relative mt-2 rounded-md shadow-sm">
                              <input v-model="session.energy" type="number" name="energy" id="energy" class="block w-full rounded-md border-0 py-1.5 pl-7 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="0.00" aria-describedby="price-currency" />
                              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
                                <span class="text-gray-500 sm:text-sm" id="price-currency">kWh</span>
                              </div>
                            </div>
                          </div>
                          <div>
                            <label for="price" class="block text-sm font-medium leading-6 text-gray-900">Odometer</label>
                            <div class="relative mt-2 rounded-md shadow-sm">
                              <input v-model="session.odometer" type="number" name="odometer" id="odometer" class="block w-full rounded-md border-0 py-1.5 pl-7 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="0.00" aria-describedby="price-currency" />
                              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
                                <span class="text-gray-500 sm:text-sm" id="price-currency">km</span>
                              </div>
                            </div>
                          </div>
                          <div>
                            <label for="price" class="block text-sm font-medium leading-6 text-gray-900">Ending State of Charge</label>
                            <div class="relative mt-2 rounded-md shadow-sm">
                              <input v-model="session.end_soc" type="number" name="end_soc" id="end_soc" class="block w-full rounded-md border-0 py-1.5 pl-7 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="0.00" aria-describedby="price-currency" />
                              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
                                <span class="text-gray-500 sm:text-sm" id="price-currency">%</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="flex flex-shrink-0 justify-end px-4 py-4">
                    <button type="button" class="rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50" @click="reset">Cancel</button>
                    <button type="button" class="ml-4 inline-flex justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600" @click="submit">Save</button>
                  </div>
                </form>
              </DialogPanel>
            </TransitionChild>
          </div>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>

<script setup>
import {Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot} from '@headlessui/vue'
import {XMarkIcon} from '@heroicons/vue/24/outline'
import {ref} from "vue";
import {SessionApi} from "../api/sessionApi.js";

const props = defineProps(['open']);
const emit = defineEmits(['toggleModal','submitSuccess'])

const format = (date) => {
  return `${date.getDate()}/${date.getMonth() + 1}/${date.getFullYear()}`
}

let session = ref({
  date: new Date(),
  energy: undefined,
  odometer: undefined,
  end_soc: undefined
});

const reset = () => {
  console.log("reset time");
  session = ref({
    date: new Date(),
    energy: undefined,
    odometer: undefined,
    end_soc: undefined
  });
  emit('toggleModal');
}

const submit = async () => {
  //todo validate
  let date = session.value.date;
  let payload = {
    date: `${date.getFullYear()}-${(date.getMonth() + 1).toString().padStart(2, "0")}-${date.getDate().toString().padStart(2, "0")}`,
    vehicle_id: 1,
    energy: session.value.energy,
    odometer: session.value.odometer,
    end_soc: session.value.end_soc
  };
  await SessionApi.addSession(1, payload);
  reset();
  emit('submitSuccess')
}
</script>