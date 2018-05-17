# Photobooth

The main photobooth frontend.
Should handle all the graphics and organize the other components like printing or taking photos.

#### Main
Where all the UI-Logic is defined.

#### Camera Layer
A wrapper module for gphoto2 camera handling and image decoding. Rustifies a lot of unsafe code to be easier to work with.