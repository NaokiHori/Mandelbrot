##########
Mandelbrot
##########

|License|_ |LastCommit|_

|QuickStart|_ |Documentation|_ |UnitTests|_

.. |License| image:: https://img.shields.io/github/license/NaokiHori/Mandelbrot
.. _License: https://opensource.org/license/MIT

.. |LastCommit| image:: https://img.shields.io/github/last-commit/NaokiHori/Mandelbrot/main
.. _LastCommit: https://github.com/NaokiHori/Mandelbrot/commits/main

.. |QuickStart| image:: https://github.com/NaokiHori/Mandelbrot/actions/workflows/quickstart.yml/badge.svg
.. _QuickStart: https://github.com/NaokiHori/Mandelbrot/actions/workflows/quickstart.yml

.. |Documentation| image:: https://github.com/NaokiHori/Mandelbrot/actions/workflows/documentation.yml/badge.svg
.. _Documentation: https://naokihori.github.io/Mandelbrot/mandelbrot/

.. |UnitTests| image:: https://github.com/NaokiHori/Mandelbrot/actions/workflows/unittests.yml/badge.svg
.. _UnitTests: https://github.com/NaokiHori/Mandelbrot/actions/workflows/unittests.yml

.. image:: https://github.com/NaokiHori/Mandelbrot/blob/artifact/image.jpg
   :target: https://youtu.be/6C9H1WNWY7s
   :width: 800

********
Overview
********

This library automatically detects complex structures in the Mandelbrot set and outputs the results as an image.

**********
Motivation
**********

This is developed

* to update my MacBook wall paper,

* to say *hello world* to Rust; in particular to create a minimal project containing file hierarchies, documentations, and tests, which could be useful for my later bigger projects.

**********
Dependency
**********

* `Cargo - the Rust package manager <https://doc.rust-lang.org/cargo/getting-started/installation.html>`_

***********
Quick start
***********

#. Prepare workplace

   .. code-block:: console

      mkdir -p /path/to/your/directory
      cd       /path/to/your/directory

#. Get source

   For example:

   .. code-block:: console

      git clone https://github.com/NaokiHori/Mandelbrot
      cd Mandelbrot

#. Build and run

   .. code-block:: console

      cargo run --release

   This takes 10 - 20 seconds.

#. Check output

   Find ``image.ppm``.

*******
Options
*******

The default behaviour can be changed by specifying several ``key-value`` pairs:

.. code-block:: console

   cargo run --release -- --<key>=<value>

All available options are listed below.

* ``seed``: random seed.

* ``grid_size``: inter-pixel distance.

* ``width``: number of pixels in the horizontal direction.

* ``height``: number of pixels in the vertical direction.

* ``fname``: name of the output image (whose suffix should be ``.ppm``).

When the initial domain does not contain any structure inside, this library aborts.
Change ``seed`` and retry if failed.

The default configuration is equivalent to

.. code-block:: console

   cargo run \
     --release \
     -- \
     --seed=0 \
     --grid_size=5.e-7 \
     --width=1280 \
     --height=800 \
     --fname=image.ppm

*********
Reference
*********

* `Mandelbrot set <https://en.wikipedia.org/wiki/Mandelbrot_set>`_

* `The Rust Programming Language <https://doc.rust-lang.org/book/>`_

* `Rust by Example <https://doc.rust-lang.org/rust-by-example/index.html>`_

* `The rustdoc book <https://doc.rust-lang.org/stable/rustdoc/>`_

