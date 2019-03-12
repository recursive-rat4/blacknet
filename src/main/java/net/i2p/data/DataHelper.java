package net.i2p.data;

/*
 * free (adj.): unencumbered; not under the control of others
 * Written by jrandom in 2003 and released into the public domain
 * with no warranty of any kind, either expressed or implied.
 * It probably won't make your computer catch on fire, or eat
 * your children, but it might.  Use at your own risk.
 *
 */

import java.io.UnsupportedEncodingException;

/**
 * Defines some simple IO routines for dealing with marshalling data structures
 *
 * @author jrandom
 */
class DataHelper {
    /**
     *  Same as orig.getBytes("UTF-8") but throws an unchecked RuntimeException
     *  instead of an UnsupportedEncodingException if no UTF-8, for ease of use.
     *
     *  @return null if orig is null
     *  @throws RuntimeException
     */
    static byte[] getUTF8(String orig) {
        if (orig == null) return null;
        try {
            return orig.getBytes("UTF-8");
        } catch (UnsupportedEncodingException uee) {
            throw new RuntimeException("no utf8!?");
        }
    }

    /**
     *  Roughly the same as orig.getBytes("ISO-8859-1") but much faster and
     *  will not throw an exception.
     *
     *  Warning - misnamed, converts to ISO-8859-1.
     *
     *  @param orig non-null, truncates to 8-bit chars
     *  @since 0.9.5
     */
    static byte[] getASCII(String orig) {
        byte[] rv = new byte[orig.length()];
        for (int i = 0; i < rv.length; i++) {
            rv[i] = (byte)orig.charAt(i);
        }
        return rv;
    }
}