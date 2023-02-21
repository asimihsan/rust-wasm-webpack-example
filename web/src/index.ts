/*
 * Copyright (C) 2023 Asim Ihsan
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU Affero General Public License as published by the Free
 * Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
 * PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with this program. If not, see <https://www.gnu.org/licenses/>
 */

import 'instant.page';

import 'bootstrap/dist/css/bootstrap.min.css';
import './index.css';
import './footer.css';
import { contentLoaded, loaded } from 'document-promises';

import importPromiseRust from './rust';
import { CounterWrapper } from './pkg_rust';

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
