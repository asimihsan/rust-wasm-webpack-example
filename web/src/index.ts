import 'instant.page';

import 'bootstrap/js/dist/button';
import 'bootstrap/js/dist/collapse';
import 'bootstrap/js/dist/dropdown';
import 'bootstrap/dist/css/bootstrap.min.css';
import './index.css';
import './footer.css';
import { contentLoaded, loaded } from 'document-promises';

import importPromiseRust from './rust';
import { CounterWrapper } from '../pkg_rust';

(() => {
    let rust;
    const contentLoadedPromise: Promise<void> = contentLoaded.then(async () => {
        rust = await importPromiseRust();
    });
    Promise.all([loaded, contentLoadedPromise]).then(() => {
        console.log('loaded');
        const counter: CounterWrapper = new rust.CounterWrapper(BigInt(0));
        console.log(counter.get_value());
        counter.increment(BigInt(1));
        console.log(counter.get_value());
    });
})();
