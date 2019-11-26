// This file was generated automagically by `gen_spdx`. Please, for the love of
// all things good, do not attempt to edit this file by hand.
//
// See instead:
// - `gen_spdx/TEMPLATE.rs`
// - `gen_spdx/src/main.rs`

use super::{BoolProperties, Info, Map};

/// A commonly found license listed [here](https://spdx.org/licenses).
///
/// This list is based on version 3.7 (2019-10-22). Please submit a pull
/// request or issue if you see that this list is out-of-date.
///
/// **SemVer Compatibility:** this license is intended to have the
/// semantics of `#[non_exhaustive]`. This library reserves the right to
/// add, reorganize, or otherwise adjust variants. These changes are
/// allowed between otherwise API-compatible versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// TODO: Add `#[non_exhaustive]` when stable
#[deny(non_camel_case_types)]
pub enum SpdxLicense {
    /// Attribution Assurance License (`AAL`).
    Aal,
    /// Abstyles License (`Abstyles`).
    Abstyles,
    /// Adobe Systems Incorporated Source Code License Agreement (`Adobe-2006`).
    Adobe2006,
    /// Adobe Glyph List License (`Adobe-Glyph`).
    AdobeGlyph,
    /// Amazon Digital Services License (`ADSL`).
    Adsl,
    /// Academic Free License v1.1 (`AFL-1.1`).
    Afl1_1,
    /// Academic Free License v1.2 (`AFL-1.2`).
    Afl1_2,
    /// Academic Free License v2.0 (`AFL-2.0`).
    Afl2,
    /// Academic Free License v2.1 (`AFL-2.1`).
    Afl2_1,
    /// Academic Free License v3.0 (`AFL-3.0`).
    Afl3,
    /// Afmparse License (`Afmparse`).
    Afmparse,
    /// Affero General Public License v1.0 (`AGPL-1.0`).
    Agpl1,
    /// Affero General Public License v1.0 or later (`AGPL-1.0-or-later`).
    Agpl1r,
    /// Affero General Public License v1.0 only (`AGPL-1.0-only`).
    Agpl1y,
    /// GNU Affero General Public License v3.0 (`AGPL-3.0`).
    Agpl3,
    /// GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`).
    Agpl3r,
    /// GNU Affero General Public License v3.0 only (`AGPL-3.0-only`).
    Agpl3y,
    /// Aladdin Free Public License (`Aladdin`).
    Aladdin,
    /// AMD's plpa_map.c License (`AMDPLPA`).
    Amdplpa,
    /// Apple MIT License (`AML`).
    Aml,
    /// Academy of Motion Picture Arts and Sciences BSD (`AMPAS`).
    Ampas,
    /// ANTLR Software Rights Notice (`ANTLR-PD`).
    AntlrPd,
    /// Apache License 1.0 (`Apache-1.0`).
    Apache1,
    /// Apache License 1.1 (`Apache-1.1`).
    Apache1_1,
    /// Apache License 2.0 (`Apache-2.0`).
    Apache2,
    /// Adobe Postscript AFM License (`APAFML`).
    Apafml,
    /// Adaptive Public License 1.0 (`APL-1.0`).
    Apl1,
    /// Apple Public Source License 1.0 (`APSL-1.0`).
    Apsl1,
    /// Apple Public Source License 1.1 (`APSL-1.1`).
    Apsl1_1,
    /// Apple Public Source License 1.2 (`APSL-1.2`).
    Apsl1_2,
    /// Apple Public Source License 2.0 (`APSL-2.0`).
    Apsl2,
    /// Artistic License 1.0 (`Artistic-1.0`).
    Artistic1,
    /// Artistic License 1.0 (Perl) (`Artistic-1.0-Perl`).
    Artistic1l,
    /// Artistic License 1.0 w/clause 8 (`Artistic-1.0-cl8`).
    Artistic1l8,
    /// Artistic License 2.0 (`Artistic-2.0`).
    Artistic2,
    /// Bahyph License (`Bahyph`).
    Bahyph,
    /// Barr License (`Barr`).
    Barr,
    /// Beerware License (`Beerware`).
    Beerware,
    /// BitTorrent Open Source License v1.0 (`BitTorrent-1.0`).
    BitTorrent1,
    /// BitTorrent Open Source License v1.1 (`BitTorrent-1.1`).
    BitTorrent1_1,
    /// SQLite Blessing (`blessing`).
    Blessing,
    /// Blue Oak Model License 1.0.0 (`BlueOak-1.0.0`).
    BlueOak1,
    /// Borceux license (`Borceux`).
    Borceux,
    /// BSD Zero Clause License (`0BSD`).
    Bsd0,
    /// BSD 1-Clause License (`BSD-1-Clause`).
    Bsd1Clause,
    /// BSD 2-Clause "Simplified" License (`BSD-2-Clause`).
    Bsd2Clause,
    /// BSD 2-Clause FreeBSD License (`BSD-2-Clause-FreeBSD`).
    Bsd2ClauseFreeBsd,
    /// BSD 2-Clause NetBSD License (`BSD-2-Clause-NetBSD`).
    Bsd2ClauseNetBsd,
    /// BSD-2-Clause Plus Patent License (`BSD-2-Clause-Patent`).
    Bsd2ClausePatent,
    /// BSD 3-Clause "New" or "Revised" License (`BSD-3-Clause`).
    Bsd3Clause,
    /// BSD with attribution (`BSD-3-Clause-Attribution`).
    Bsd3ClauseAttribution,
    /// BSD 3-Clause Clear License (`BSD-3-Clause-Clear`).
    Bsd3ClauseClear,
    /// Lawrence Berkeley National Labs BSD variant license (`BSD-3-Clause-LBNL`).
    Bsd3ClauseLbnl,
    /// BSD 3-Clause No Nuclear License (`BSD-3-Clause-No-Nuclear-License`).
    Bsd3ClauseNoNuclearLicense,
    /// BSD 3-Clause No Nuclear License 2014 (`BSD-3-Clause-No-Nuclear-License-2014`).
    Bsd3ClauseNoNuclearLicense2014,
    /// BSD 3-Clause No Nuclear Warranty (`BSD-3-Clause-No-Nuclear-Warranty`).
    Bsd3ClauseNoNuclearWarranty,
    /// BSD 3-Clause Open MPI variant (`BSD-3-Clause-Open-MPI`).
    Bsd3ClauseOpenMpi,
    /// BSD 4-Clause "Original" or "Old" License (`BSD-4-Clause`).
    Bsd4Clause,
    /// BSD-4-Clause (University of California-Specific) (`BSD-4-Clause-UC`).
    Bsd4ClauseUc,
    /// BSD Protection License (`BSD-Protection`).
    BsdProtection,
    /// BSD Source Code Attribution (`BSD-Source-Code`).
    BsdSourceCode,
    /// Boost Software License 1.0 (`BSL-1.0`).
    Bsl1,
    /// bzip2 and libbzip2 License v1.0.5 (`bzip2-1.0.5`).
    Bzip2_1_5,
    /// bzip2 and libbzip2 License v1.0.6 (`bzip2-1.0.6`).
    Bzip2_1_6,
    /// Caldera License (`Caldera`).
    Caldera,
    /// Computer Associates Trusted Open Source License 1.1 (`CATOSL-1.1`).
    Catosl1_1,
    /// Creative Commons Zero v1.0 Universal (`CC0-1.0`).
    Cc0_1,
    /// Creative Commons Attribution 1.0 Generic (`CC-BY-1.0`).
    CcBy1,
    /// Creative Commons Attribution 2.0 Generic (`CC-BY-2.0`).
    CcBy2,
    /// Creative Commons Attribution 2.5 Generic (`CC-BY-2.5`).
    CcBy2_5,
    /// Creative Commons Attribution 3.0 Unported (`CC-BY-3.0`).
    CcBy3,
    /// Creative Commons Attribution 4.0 International (`CC-BY-4.0`).
    CcBy4,
    /// Creative Commons Attribution Non Commercial 1.0 Generic (`CC-BY-NC-1.0`).
    CcByNc1,
    /// Creative Commons Attribution Non Commercial 2.0 Generic (`CC-BY-NC-2.0`).
    CcByNc2,
    /// Creative Commons Attribution Non Commercial 2.5 Generic (`CC-BY-NC-2.5`).
    CcByNc2_5,
    /// Creative Commons Attribution Non Commercial 3.0 Unported (`CC-BY-NC-3.0`).
    CcByNc3,
    /// Creative Commons Attribution Non Commercial 4.0 International (`CC-BY-NC-4.0`).
    CcByNc4,
    /// Creative Commons Attribution Non Commercial No Derivatives 1.0 Generic (`CC-BY-NC-ND-1.0`).
    CcByNcNd1,
    /// Creative Commons Attribution Non Commercial No Derivatives 2.0 Generic (`CC-BY-NC-ND-2.0`).
    CcByNcNd2,
    /// Creative Commons Attribution Non Commercial No Derivatives 2.5 Generic (`CC-BY-NC-ND-2.5`).
    CcByNcNd2_5,
    /// Creative Commons Attribution Non Commercial No Derivatives 3.0 Unported (`CC-BY-NC-ND-3.0`).
    CcByNcNd3,
    /// Creative Commons Attribution Non Commercial No Derivatives 4.0 International (`CC-BY-NC-ND-4.0`).
    CcByNcNd4,
    /// Creative Commons Attribution Non Commercial Share Alike 1.0 Generic (`CC-BY-NC-SA-1.0`).
    CcByNcSa1,
    /// Creative Commons Attribution Non Commercial Share Alike 2.0 Generic (`CC-BY-NC-SA-2.0`).
    CcByNcSa2,
    /// Creative Commons Attribution Non Commercial Share Alike 2.5 Generic (`CC-BY-NC-SA-2.5`).
    CcByNcSa2_5,
    /// Creative Commons Attribution Non Commercial Share Alike 3.0 Unported (`CC-BY-NC-SA-3.0`).
    CcByNcSa3,
    /// Creative Commons Attribution Non Commercial Share Alike 4.0 International (`CC-BY-NC-SA-4.0`).
    CcByNcSa4,
    /// Creative Commons Attribution No Derivatives 1.0 Generic (`CC-BY-ND-1.0`).
    CcByNd1,
    /// Creative Commons Attribution No Derivatives 2.0 Generic (`CC-BY-ND-2.0`).
    CcByNd2,
    /// Creative Commons Attribution No Derivatives 2.5 Generic (`CC-BY-ND-2.5`).
    CcByNd2_5,
    /// Creative Commons Attribution No Derivatives 3.0 Unported (`CC-BY-ND-3.0`).
    CcByNd3,
    /// Creative Commons Attribution No Derivatives 4.0 International (`CC-BY-ND-4.0`).
    CcByNd4,
    /// Creative Commons Attribution Share Alike 1.0 Generic (`CC-BY-SA-1.0`).
    CcBySa1,
    /// Creative Commons Attribution Share Alike 2.0 Generic (`CC-BY-SA-2.0`).
    CcBySa2,
    /// Creative Commons Attribution Share Alike 2.5 Generic (`CC-BY-SA-2.5`).
    CcBySa2_5,
    /// Creative Commons Attribution Share Alike 3.0 Unported (`CC-BY-SA-3.0`).
    CcBySa3,
    /// Creative Commons Attribution Share Alike 4.0 International (`CC-BY-SA-4.0`).
    CcBySa4,
    /// Creative Commons Public Domain Dedication and Certification (`CC-PDDC`).
    CcPddc,
    /// Common Development and Distribution License 1.0 (`CDDL-1.0`).
    Cddl1,
    /// Common Development and Distribution License 1.1 (`CDDL-1.1`).
    Cddl1_1,
    /// Community Data License Agreement Permissive 1.0 (`CDLA-Permissive-1.0`).
    CdlaPermissive1,
    /// Community Data License Agreement Sharing 1.0 (`CDLA-Sharing-1.0`).
    CdlaSharing1,
    /// CeCILL Free Software License Agreement v1.0 (`CECILL-1.0`).
    Cecill1,
    /// CeCILL Free Software License Agreement v1.1 (`CECILL-1.1`).
    Cecill1_1,
    /// CeCILL Free Software License Agreement v2.0 (`CECILL-2.0`).
    Cecill2,
    /// CeCILL Free Software License Agreement v2.1 (`CECILL-2.1`).
    Cecill2_1,
    /// CeCILL-B Free Software License Agreement (`CECILL-B`).
    CecillB,
    /// CeCILL-C Free Software License Agreement (`CECILL-C`).
    CecillC,
    /// CERN Open Hardware Licence v1.1 (`CERN-OHL-1.1`).
    CernOhl1_1,
    /// CERN Open Hardware Licence v1.2 (`CERN-OHL-1.2`).
    CernOhl1_2,
    /// Clarified Artistic License (`ClArtistic`).
    ClArtistic,
    /// CNRI Jython License (`CNRI-Jython`).
    CnriJython,
    /// CNRI Python License (`CNRI-Python`).
    CnriPython,
    /// CNRI Python Open Source GPL Compatible License Agreement (`CNRI-Python-GPL-Compatible`).
    CnriPythonGplCompatible,
    /// Condor Public License v1.1 (`Condor-1.1`).
    Condor1_1,
    /// copyleft-next 0.3.0 (`copyleft-next-0.3.0`).
    CopyleftNext0_3,
    /// copyleft-next 0.3.1 (`copyleft-next-0.3.1`).
    CopyleftNext0_3_1,
    /// Common Public Attribution License 1.0 (`CPAL-1.0`).
    Cpal1,
    /// Common Public License 1.0 (`CPL-1.0`).
    Cpl1,
    /// Code Project Open License 1.02 (`CPOL-1.02`).
    Cpol12,
    /// Crossword License (`Crossword`).
    Crossword,
    /// CrystalStacker License (`CrystalStacker`).
    CrystalStacker,
    /// CUA Office Public License v1.0 (`CUA-OPL-1.0`).
    CuaOpl1,
    /// Cube License (`Cube`).
    Cube,
    /// curl License (`curl`).
    Curl,
    /// Deutsche Freie Software Lizenz (`D-FSL-1.0`).
    DFsl1,
    /// diffmark license (`diffmark`).
    Diffmark,
    /// DOC License (`DOC`).
    Doc,
    /// Dotseqn License (`Dotseqn`).
    Dotseqn,
    /// DSDP License (`DSDP`).
    Dsdp,
    /// dvipdfm License (`dvipdfm`).
    Dvipdfm,
    /// eCos license version 2.0 (`eCos-2.0`).
    ECos2,
    /// eGenix.com Public License 1.1.0 (`eGenix`).
    EGenix,
    /// Educational Community License v1.0 (`ECL-1.0`).
    Ecl1,
    /// Educational Community License v2.0 (`ECL-2.0`).
    Ecl2,
    /// Eiffel Forum License v1.0 (`EFL-1.0`).
    Efl1,
    /// Eiffel Forum License v2.0 (`EFL-2.0`).
    Efl2,
    /// Entessa Public License v1.0 (`Entessa`).
    Entessa,
    /// Eclipse Public License 1.0 (`EPL-1.0`).
    Epl1,
    /// Eclipse Public License 2.0 (`EPL-2.0`).
    Epl2,
    /// Erlang Public License v1.1 (`ErlPL-1.1`).
    ErlPl1_1,
    /// Etalab Open License 2.0 (`etalab-2.0`).
    Etalab2,
    /// EU DataGrid Software License (`EUDatagrid`).
    Eudatagrid,
    /// European Union Public License 1.0 (`EUPL-1.0`).
    Eupl1,
    /// European Union Public License 1.1 (`EUPL-1.1`).
    Eupl1_1,
    /// European Union Public License 1.2 (`EUPL-1.2`).
    Eupl1_2,
    /// Eurosym License (`Eurosym`).
    Eurosym,
    /// Fair License (`Fair`).
    Fair,
    /// Frameworx Open License 1.0 (`Frameworx-1.0`).
    Frameworx1,
    /// FreeImage Public License v1.0 (`FreeImage`).
    FreeImage,
    /// FSF All Permissive License (`FSFAP`).
    Fsfap,
    /// FSF Unlimited License (`FSFUL`).
    Fsful,
    /// FSF Unlimited License (with License Retention) (`FSFULLR`).
    Fsfullr,
    /// Freetype Project License (`FTL`).
    Ftl,
    /// gSOAP Public License v1.3b (`gSOAP-1.3b`).
    GSoap1_3b,
    /// GNU Free Documentation License v1.1 (`GFDL-1.1`).
    Gfdl1_1,
    /// GNU Free Documentation License v1.1 only (`GFDL-1.1-only`).
    Gfdl1_1Only,
    /// GNU Free Documentation License v1.1 or later (`GFDL-1.1-or-later`).
    Gfdl1_1OrLater,
    /// GNU Free Documentation License v1.2 (`GFDL-1.2`).
    Gfdl1_2,
    /// GNU Free Documentation License v1.2 only (`GFDL-1.2-only`).
    Gfdl1_2Only,
    /// GNU Free Documentation License v1.2 or later (`GFDL-1.2-or-later`).
    Gfdl1_2OrLater,
    /// GNU Free Documentation License v1.3 (`GFDL-1.3`).
    Gfdl1_3,
    /// GNU Free Documentation License v1.3 only (`GFDL-1.3-only`).
    Gfdl1_3Only,
    /// GNU Free Documentation License v1.3 or later (`GFDL-1.3-or-later`).
    Gfdl1_3OrLater,
    /// Giftware License (`Giftware`).
    Giftware,
    /// GL2PS License (`GL2PS`).
    Gl2Ps,
    /// 3dfx Glide License (`Glide`).
    Glide,
    /// Glulxe License (`Glulxe`).
    Glulxe,
    /// gnuplot License (`gnuplot`).
    Gnuplot,
    /// GNU General Public License v1.0 only (`GPL-1.0`).
    Gpl1,
    /// GNU General Public License v1.0 or later (`GPL-1.0+`).
    Gpl1Plus,
    /// GNU General Public License v1.0 or later (`GPL-1.0-or-later`).
    Gpl1r,
    /// GNU General Public License v1.0 only (`GPL-1.0-only`).
    Gpl1y,
    /// GNU General Public License v2.0 only (`GPL-2.0`).
    Gpl2,
    /// GNU General Public License v2.0 or later (`GPL-2.0+`).
    Gpl2Plus,
    /// GNU General Public License v2.0 w/Autoconf exception (`GPL-2.0-with-autoconf-exception`).
    Gpl2n,
    /// GNU General Public License v2.0 w/Bison exception (`GPL-2.0-with-bison-exception`).
    Gpl2n,
    /// GNU General Public License v2.0 w/Classpath exception (`GPL-2.0-with-classpath-exception`).
    Gpl2n,
    /// GNU General Public License v2.0 w/Font exception (`GPL-2.0-with-font-exception`).
    Gpl2n,
    /// GNU General Public License v2.0 w/GCC Runtime Library exception (`GPL-2.0-with-GCC-exception`).
    Gpl2n,
    /// GNU General Public License v2.0 or later (`GPL-2.0-or-later`).
    Gpl2r,
    /// GNU General Public License v2.0 only (`GPL-2.0-only`).
    Gpl2y,
    /// GNU General Public License v3.0 only (`GPL-3.0`).
    Gpl3,
    /// GNU General Public License v3.0 or later (`GPL-3.0+`).
    Gpl3Plus,
    /// GNU General Public License v3.0 w/GCC Runtime Library exception (`GPL-3.0-with-GCC-exception`).
    Gpl3n,
    /// GNU General Public License v3.0 w/Autoconf exception (`GPL-3.0-with-autoconf-exception`).
    Gpl3n,
    /// GNU General Public License v3.0 or later (`GPL-3.0-or-later`).
    Gpl3r,
    /// GNU General Public License v3.0 only (`GPL-3.0-only`).
    Gpl3y,
    /// Haskell Language Report License (`HaskellReport`).
    HaskellReport,
    /// Historical Permission Notice and Disclaimer (`HPND`).
    Hpnd,
    /// Historical Permission Notice and Disclaimer - sell variant (`HPND-sell-variant`).
    HpndSellVariant,
    /// iMatix Standard Function Library Agreement (`iMatix`).
    IMatix,
    /// IBM PowerPC Initialization and Boot Software (`IBM-pibs`).
    IbmPibs,
    /// ICU License (`ICU`).
    Icu,
    /// Independent JPEG Group License (`IJG`).
    Ijg,
    /// ImageMagick License (`ImageMagick`).
    ImageMagick,
    /// Imlib2 License (`Imlib2`).
    Imlib2,
    /// Info-ZIP License (`Info-ZIP`).
    InfoZip,
    /// Intel Open Source License (`Intel`).
    Intel,
    /// Intel ACPI Software License Agreement (`Intel-ACPI`).
    IntelAcpi,
    /// Interbase Public License v1.0 (`Interbase-1.0`).
    Interbase1,
    /// IPA Font License (`IPA`).
    Ipa,
    /// IBM Public License v1.0 (`IPL-1.0`).
    Ipl1,
    /// ISC License (`ISC`).
    Isc,
    /// JasPer License (`JasPer-2.0`).
    JasPer2,
    /// Japan Network Information Center License (`JPNIC`).
    Jpnic,
    /// JSON License (`JSON`).
    Json,
    /// Licence Art Libre 1.2 (`LAL-1.2`).
    Lal1_2,
    /// Licence Art Libre 1.3 (`LAL-1.3`).
    Lal1_3,
    /// Latex2e License (`Latex2e`).
    Latex2e,
    /// Leptonica License (`Leptonica`).
    Leptonica,
    /// GNU Library General Public License v2 only (`LGPL-2.0`).
    Lgpl2,
    /// GNU Library General Public License v2 or later (`LGPL-2.0+`).
    Lgpl2Plus,
    /// GNU Lesser General Public License v2.1 only (`LGPL-2.1`).
    Lgpl2_1,
    /// GNU Lesser General Public License v2.1 only (`LGPL-2.1-only`).
    Lgpl2_1Only,
    /// GNU Lesser General Public License v2.1 or later (`LGPL-2.1-or-later`).
    Lgpl2_1OrLater,
    /// GNU Library General Public License v2.1 or later (`LGPL-2.1+`).
    Lgpl2_1Plus,
    /// GNU Library General Public License v2 or later (`LGPL-2.0-or-later`).
    Lgpl2r,
    /// GNU Library General Public License v2 only (`LGPL-2.0-only`).
    Lgpl2y,
    /// GNU Lesser General Public License v3.0 only (`LGPL-3.0`).
    Lgpl3,
    /// GNU Lesser General Public License v3.0 or later (`LGPL-3.0+`).
    Lgpl3Plus,
    /// GNU Lesser General Public License v3.0 or later (`LGPL-3.0-or-later`).
    Lgpl3r,
    /// GNU Lesser General Public License v3.0 only (`LGPL-3.0-only`).
    Lgpl3y,
    /// Lesser General Public License For Linguistic Resources (`LGPLLR`).
    Lgpllr,
    /// Licence Libre du Québec – Permissive version 1.1 (`LiLiQ-P-1.1`).
    LiLiQP1_1,
    /// Licence Libre du Québec – Réciprocité version 1.1 (`LiLiQ-R-1.1`).
    LiLiQR1_1,
    /// Licence Libre du Québec – Réciprocité forte version 1.1 (`LiLiQ-Rplus-1.1`).
    LiLiQRplus1_1,
    /// libpng License (`Libpng`).
    Libpng,
    /// PNG Reference Library version 2 (`libpng-2.0`).
    Libpng2,
    /// libtiff License (`libtiff`).
    Libtiff,
    /// Linux Kernel Variant of OpenIB.org license (`Linux-OpenIB`).
    LinuxOpenIb,
    /// Lucent Public License Version 1.0 (`LPL-1.0`).
    Lpl1,
    /// Lucent Public License v1.02 (`LPL-1.02`).
    Lpl12,
    /// LaTeX Project Public License v1.0 (`LPPL-1.0`).
    Lppl1,
    /// LaTeX Project Public License v1.1 (`LPPL-1.1`).
    Lppl1_1,
    /// LaTeX Project Public License v1.2 (`LPPL-1.2`).
    Lppl1_2,
    /// LaTeX Project Public License v1.3a (`LPPL-1.3a`).
    Lppl1_3a,
    /// LaTeX Project Public License v1.3c (`LPPL-1.3c`).
    Lppl1_3c,
    /// MakeIndex License (`MakeIndex`).
    MakeIndex,
    /// The MirOS Licence (`MirOS`).
    MirOs,
    /// MIT License (`MIT`).
    Mit,
    /// MIT No Attribution (`MIT-0`).
    Mit0,
    /// Enlightenment License (e16) (`MIT-advertising`).
    MitAdvertising,
    /// CMU License (`MIT-CMU`).
    MitCmu,
    /// enna License (`MIT-enna`).
    MitEnna,
    /// feh License (`MIT-feh`).
    MitFeh,
    /// MIT +no-false-attribs license (`MITNFA`).
    Mitnfa,
    /// Motosoto License (`Motosoto`).
    Motosoto,
    /// mpich2 License (`mpich2`).
    Mpich2,
    /// Mozilla Public License 1.0 (`MPL-1.0`).
    Mpl1,
    /// Mozilla Public License 1.1 (`MPL-1.1`).
    Mpl1_1,
    /// Mozilla Public License 2.0 (`MPL-2.0`).
    Mpl2,
    /// Mozilla Public License 2.0 (no copyleft exception) (`MPL-2.0-no-copyleft-exception`).
    Mpl2n,
    /// Microsoft Public License (`MS-PL`).
    MsPl,
    /// Microsoft Reciprocal License (`MS-RL`).
    MsRl,
    /// Matrix Template Library License (`MTLL`).
    Mtll,
    /// Mulan Permissive Software License, Version 1 (`MulanPSL-1.0`).
    MulanPsl1,
    /// Multics License (`Multics`).
    Multics,
    /// Mup License (`Mup`).
    Mup,
    /// NASA Open Source Agreement 1.3 (`NASA-1.3`).
    Nasa1_3,
    /// Naumen Public License (`Naumen`).
    Naumen,
    /// Net Boolean Public License v1 (`NBPL-1.0`).
    Nbpl1,
    /// University of Illinois/NCSA Open Source License (`NCSA`).
    Ncsa,
    /// NetCDF license (`NetCDF`).
    NetCdf,
    /// Net-SNMP License (`Net-SNMP`).
    NetSnmp,
    /// Newsletr License (`Newsletr`).
    Newsletr,
    /// Nethack General Public License (`NGPL`).
    Ngpl,
    /// Norwegian Licence for Open Government Data (`NLOD-1.0`).
    Nlod1,
    /// No Limit Public License (`NLPL`).
    Nlpl,
    /// Nokia Open Source License (`Nokia`).
    Nokia,
    /// Netizen Open Source License (`NOSL`).
    Nosl,
    /// Noweb License (`Noweb`).
    Noweb,
    /// Netscape Public License v1.0 (`NPL-1.0`).
    Npl1,
    /// Netscape Public License v1.1 (`NPL-1.1`).
    Npl1_1,
    /// Non-Profit Open Software License 3.0 (`NPOSL-3.0`).
    Nposl3,
    /// NRL License (`NRL`).
    Nrl,
    /// NTP License (`NTP`).
    Ntp,
    /// Nunit License (`Nunit`).
    Nunit,
    /// Open CASCADE Technology Public License (`OCCT-PL`).
    OcctPl,
    /// OCLC Research Public License 2.0 (`OCLC-2.0`).
    Oclc2,
    /// ODC Open Database License v1.0 (`ODbL-1.0`).
    OdbL1,
    /// Open Data Commons Attribution License v1.0 (`ODC-By-1.0`).
    OdcBy1,
    /// SIL Open Font License 1.0 (`OFL-1.0`).
    Ofl1,
    /// SIL Open Font License 1.1 (`OFL-1.1`).
    Ofl1_1,
    /// Open Government Licence - Canada (`OGL-Canada-2.0`).
    OglCanada2,
    /// Open Government Licence v1.0 (`OGL-UK-1.0`).
    OglUk1,
    /// Open Government Licence v2.0 (`OGL-UK-2.0`).
    OglUk2,
    /// Open Government Licence v3.0 (`OGL-UK-3.0`).
    OglUk3,
    /// Open Group Test Suite License (`OGTSL`).
    Ogtsl,
    /// Open LDAP Public License v1.1 (`OLDAP-1.1`).
    Oldap1_1,
    /// Open LDAP Public License v1.2 (`OLDAP-1.2`).
    Oldap1_2,
    /// Open LDAP Public License v1.3 (`OLDAP-1.3`).
    Oldap1_3,
    /// Open LDAP Public License v1.4 (`OLDAP-1.4`).
    Oldap1_4,
    /// Open LDAP Public License v2.0 (or possibly 2.0A and 2.0B) (`OLDAP-2.0`).
    Oldap2,
    /// Open LDAP Public License v2.0.1 (`OLDAP-2.0.1`).
    Oldap2_1,
    /// Open LDAP Public License v2.1 (`OLDAP-2.1`).
    Oldap2_1,
    /// Open LDAP Public License v2.2 (`OLDAP-2.2`).
    Oldap2_2,
    /// Open LDAP Public License v2.2.1 (`OLDAP-2.2.1`).
    Oldap2_2_1,
    /// Open LDAP Public License 2.2.2 (`OLDAP-2.2.2`).
    Oldap2_2_2,
    /// Open LDAP Public License v2.3 (`OLDAP-2.3`).
    Oldap2_3,
    /// Open LDAP Public License v2.4 (`OLDAP-2.4`).
    Oldap2_4,
    /// Open LDAP Public License v2.5 (`OLDAP-2.5`).
    Oldap2_5,
    /// Open LDAP Public License v2.6 (`OLDAP-2.6`).
    Oldap2_6,
    /// Open LDAP Public License v2.7 (`OLDAP-2.7`).
    Oldap2_7,
    /// Open LDAP Public License v2.8 (`OLDAP-2.8`).
    Oldap2_8,
    /// Open Market License (`OML`).
    Oml,
    /// OpenSSL License (`OpenSSL`).
    OpenSsl,
    /// Open Public License v1.0 (`OPL-1.0`).
    Opl1,
    /// OSET Public License version 2.1 (`OSET-PL-2.1`).
    OsetPl2_1,
    /// Open Software License 1.0 (`OSL-1.0`).
    Osl1,
    /// Open Software License 1.1 (`OSL-1.1`).
    Osl1_1,
    /// Open Software License 2.0 (`OSL-2.0`).
    Osl2,
    /// Open Software License 2.1 (`OSL-2.1`).
    Osl2_1,
    /// Open Software License 3.0 (`OSL-3.0`).
    Osl3,
    /// The Parity Public License 6.0.0 (`Parity-6.0.0`).
    Parity6,
    /// ODC Public Domain Dedication & License 1.0 (`PDDL-1.0`).
    Pddl1,
    /// PHP License v3.0 (`PHP-3.0`).
    Php3,
    /// PHP License v3.01 (`PHP-3.01`).
    Php31,
    /// Plexus Classworlds License (`Plexus`).
    Plexus,
    /// PostgreSQL License (`PostgreSQL`).
    PostgreSql,
    /// psfrag License (`psfrag`).
    Psfrag,
    /// psutils License (`psutils`).
    Psutils,
    /// Python License 2.0 (`Python-2.0`).
    Python2,
    /// Qhull License (`Qhull`).
    Qhull,
    /// Q Public License 1.0 (`QPL-1.0`).
    Qpl1,
    /// Rdisc License (`Rdisc`).
    Rdisc,
    /// Red Hat eCos Public License v1.1 (`RHeCos-1.1`).
    RheCos1_1,
    /// Reciprocal Public License 1.1 (`RPL-1.1`).
    Rpl1_1,
    /// Reciprocal Public License 1.5 (`RPL-1.5`).
    Rpl1_5,
    /// RealNetworks Public Source License v1.0 (`RPSL-1.0`).
    Rpsl1,
    /// RSA Message-Digest License  (`RSA-MD`).
    RsaMd,
    /// Ricoh Source Code Public License (`RSCPL`).
    Rscpl,
    /// Ruby License (`Ruby`).
    Ruby,
    /// Sax Public Domain Notice (`SAX-PD`).
    SaxPd,
    /// Saxpath License (`Saxpath`).
    Saxpath,
    /// SCEA Shared Source License (`SCEA`).
    Scea,
    /// Sendmail License (`Sendmail`).
    Sendmail,
    /// Sendmail License 8.23 (`Sendmail-8.23`).
    Sendmail8_23,
    /// SGI Free Software License B v1.0 (`SGI-B-1.0`).
    SgiB1,
    /// SGI Free Software License B v1.1 (`SGI-B-1.1`).
    SgiB1_1,
    /// SGI Free Software License B v2.0 (`SGI-B-2.0`).
    SgiB2,
    /// Solderpad Hardware License v0.5 (`SHL-0.5`).
    Shl0_5,
    /// Solderpad Hardware License, Version 0.51 (`SHL-0.51`).
    Shl0_51,
    /// Simple Public License 2.0 (`SimPL-2.0`).
    SimPl2,
    /// Sun Industry Standards Source License v1.1 (`SISSL`).
    Sissl,
    /// Sun Industry Standards Source License v1.2 (`SISSL-1.2`).
    Sissl1_2,
    /// Sleepycat License (`Sleepycat`).
    Sleepycat,
    /// Standard ML of New Jersey License (`SMLNJ`).
    Smlnj,
    /// Secure Messaging Protocol Public License (`SMPPL`).
    Smppl,
    /// SNIA Public License 1.1 (`SNIA`).
    Snia,
    /// Spencer License 86 (`Spencer-86`).
    Spencer86,
    /// Spencer License 94 (`Spencer-94`).
    Spencer94,
    /// Spencer License 99 (`Spencer-99`).
    Spencer99,
    /// Sun Public License v1.0 (`SPL-1.0`).
    Spl1,
    /// SSH OpenSSH license (`SSH-OpenSSH`).
    SshOpenSsh,
    /// SSH short notice (`SSH-short`).
    SshShort,
    /// Server Side Public License, v 1 (`SSPL-1.0`).
    Sspl1,
    /// Standard ML of New Jersey License (`StandardML-NJ`).
    StandardMlNj,
    /// SugarCRM Public License v1.1.3 (`SugarCRM-1.1.3`).
    SugarCrm1_1_3,
    /// Scheme Widget Library (SWL) Software License Agreement (`SWL`).
    Swl,
    /// TAPR Open Hardware License v1.0 (`TAPR-OHL-1.0`).
    TaprOhl1,
    /// TCL/TK License (`TCL`).
    Tcl,
    /// TCP Wrappers License (`TCP-wrappers`).
    TcpWrappers,
    /// TMate Open Source License (`TMate`).
    Tmate,
    /// TORQUE v2.5+ Software License v1.1 (`TORQUE-1.1`).
    Torque1_1,
    /// Trusster Open Source License (`TOSL`).
    Tosl,
    /// Technische Universitaet Berlin License 1.0 (`TU-Berlin-1.0`).
    TuBerlin1,
    /// Technische Universitaet Berlin License 2.0 (`TU-Berlin-2.0`).
    TuBerlin2,
    /// Upstream Compatibility License v1.0 (`UCL-1.0`).
    Ucl1,
    /// Unicode License Agreement - Data Files and Software (2015) (`Unicode-DFS-2015`).
    UnicodeDfs2015,
    /// Unicode License Agreement - Data Files and Software (2016) (`Unicode-DFS-2016`).
    UnicodeDfs2016,
    /// Unicode Terms of Use (`Unicode-TOU`).
    UnicodeTou,
    /// The Unlicense (`Unlicense`).
    Unlicense,
    /// Universal Permissive License v1.0 (`UPL-1.0`).
    Upl1,
    /// Vim License (`Vim`).
    Vim,
    /// VOSTROM Public License for Open Source (`VOSTROM`).
    Vostrom,
    /// Vovida Software License v1.0 (`VSL-1.0`).
    Vsl1,
    /// W3C Software Notice and License (2002-12-31) (`W3C`).
    W3C,
    /// W3C Software Notice and License (1998-07-20) (`W3C-19980720`).
    W3C19980720,
    /// W3C Software Notice and Document License (2015-05-13) (`W3C-20150513`).
    W3C20150513,
    /// Sybase Open Watcom Public License 1.0 (`Watcom-1.0`).
    Watcom1,
    /// Wsuipa License (`Wsuipa`).
    Wsuipa,
    /// Do What The F*ck You Want To Public License (`WTFPL`).
    Wtfpl,
    /// wxWindows Library License (`wxWindows`).
    WxWindows,
    /// X11 License (`X11`).
    X11,
    /// Xerox License (`Xerox`).
    Xerox,
    /// XFree86 License 1.1 (`XFree86-1.1`).
    Xfree86_1_1,
    /// xinetd License (`xinetd`).
    Xinetd,
    /// X.Net License (`Xnet`).
    Xnet,
    /// XPP License (`xpp`).
    Xpp,
    /// XSkat License (`XSkat`).
    Xskat,
    /// Yahoo! Public License v1.0 (`YPL-1.0`).
    Ypl1,
    /// Yahoo! Public License v1.1 (`YPL-1.1`).
    Ypl1_1,
    /// Zed License (`Zed`).
    Zed,
    /// Zend License v2.0 (`Zend-2.0`).
    Zend2,
    /// Zimbra Public License v1.3 (`Zimbra-1.3`).
    Zimbra1_3,
    /// Zimbra Public License v1.4 (`Zimbra-1.4`).
    Zimbra1_4,
    /// zlib License (`Zlib`).
    Zlib,
    /// zlib/libpng License with Acknowledgement (`zlib-acknowledgement`).
    ZlibAcknowledgement,
    /// Zope Public License 1.1 (`ZPL-1.1`).
    Zpl1_1,
    /// Zope Public License 2.0 (`ZPL-2.0`).
    Zpl2,
    /// Zope Public License 2.1 (`ZPL-2.1`).
    Zpl2_1,
}

impl SpdxLicense {
    // Defined here to make use of macro repetition; the public API is `COUNT`
    // which is declared with the other public items.
    pub(crate) const _COUNT: usize = 402;

    pub(crate) const _INFO: Info = Info {
        ids_with_names: [
            (r#"AAL"#, r#"Attribution Assurance License"#),
            (r#"Abstyles"#, r#"Abstyles License"#),
            (
                r#"Adobe-2006"#,
                r#"Adobe Systems Incorporated Source Code License Agreement"#,
            ),
            (r#"Adobe-Glyph"#, r#"Adobe Glyph List License"#),
            (r#"ADSL"#, r#"Amazon Digital Services License"#),
            (r#"AFL-1.1"#, r#"Academic Free License v1.1"#),
            (r#"AFL-1.2"#, r#"Academic Free License v1.2"#),
            (r#"AFL-2.0"#, r#"Academic Free License v2.0"#),
            (r#"AFL-2.1"#, r#"Academic Free License v2.1"#),
            (r#"AFL-3.0"#, r#"Academic Free License v3.0"#),
            (r#"Afmparse"#, r#"Afmparse License"#),
            (r#"AGPL-1.0"#, r#"Affero General Public License v1.0"#),
            (
                r#"AGPL-1.0-or-later"#,
                r#"Affero General Public License v1.0 or later"#,
            ),
            (
                r#"AGPL-1.0-only"#,
                r#"Affero General Public License v1.0 only"#,
            ),
            (r#"AGPL-3.0"#, r#"GNU Affero General Public License v3.0"#),
            (
                r#"AGPL-3.0-or-later"#,
                r#"GNU Affero General Public License v3.0 or later"#,
            ),
            (
                r#"AGPL-3.0-only"#,
                r#"GNU Affero General Public License v3.0 only"#,
            ),
            (r#"Aladdin"#, r#"Aladdin Free Public License"#),
            (r#"AMDPLPA"#, r#"AMD's plpa_map.c License"#),
            (r#"AML"#, r#"Apple MIT License"#),
            (
                r#"AMPAS"#,
                r#"Academy of Motion Picture Arts and Sciences BSD"#,
            ),
            (r#"ANTLR-PD"#, r#"ANTLR Software Rights Notice"#),
            (r#"Apache-1.0"#, r#"Apache License 1.0"#),
            (r#"Apache-1.1"#, r#"Apache License 1.1"#),
            (r#"Apache-2.0"#, r#"Apache License 2.0"#),
            (r#"APAFML"#, r#"Adobe Postscript AFM License"#),
            (r#"APL-1.0"#, r#"Adaptive Public License 1.0"#),
            (r#"APSL-1.0"#, r#"Apple Public Source License 1.0"#),
            (r#"APSL-1.1"#, r#"Apple Public Source License 1.1"#),
            (r#"APSL-1.2"#, r#"Apple Public Source License 1.2"#),
            (r#"APSL-2.0"#, r#"Apple Public Source License 2.0"#),
            (r#"Artistic-1.0"#, r#"Artistic License 1.0"#),
            (r#"Artistic-1.0-Perl"#, r#"Artistic License 1.0 (Perl)"#),
            (r#"Artistic-1.0-cl8"#, r#"Artistic License 1.0 w/clause 8"#),
            (r#"Artistic-2.0"#, r#"Artistic License 2.0"#),
            (r#"Bahyph"#, r#"Bahyph License"#),
            (r#"Barr"#, r#"Barr License"#),
            (r#"Beerware"#, r#"Beerware License"#),
            (
                r#"BitTorrent-1.0"#,
                r#"BitTorrent Open Source License v1.0"#,
            ),
            (
                r#"BitTorrent-1.1"#,
                r#"BitTorrent Open Source License v1.1"#,
            ),
            (r#"blessing"#, r#"SQLite Blessing"#),
            (r#"BlueOak-1.0.0"#, r#"Blue Oak Model License 1.0.0"#),
            (r#"Borceux"#, r#"Borceux license"#),
            (r#"0BSD"#, r#"BSD Zero Clause License"#),
            (r#"BSD-1-Clause"#, r#"BSD 1-Clause License"#),
            (r#"BSD-2-Clause"#, r#"BSD 2-Clause "Simplified" License"#),
            (r#"BSD-2-Clause-FreeBSD"#, r#"BSD 2-Clause FreeBSD License"#),
            (r#"BSD-2-Clause-NetBSD"#, r#"BSD 2-Clause NetBSD License"#),
            (
                r#"BSD-2-Clause-Patent"#,
                r#"BSD-2-Clause Plus Patent License"#,
            ),
            (
                r#"BSD-3-Clause"#,
                r#"BSD 3-Clause "New" or "Revised" License"#,
            ),
            (r#"BSD-3-Clause-Attribution"#, r#"BSD with attribution"#),
            (r#"BSD-3-Clause-Clear"#, r#"BSD 3-Clause Clear License"#),
            (
                r#"BSD-3-Clause-LBNL"#,
                r#"Lawrence Berkeley National Labs BSD variant license"#,
            ),
            (
                r#"BSD-3-Clause-No-Nuclear-License"#,
                r#"BSD 3-Clause No Nuclear License"#,
            ),
            (
                r#"BSD-3-Clause-No-Nuclear-License-2014"#,
                r#"BSD 3-Clause No Nuclear License 2014"#,
            ),
            (
                r#"BSD-3-Clause-No-Nuclear-Warranty"#,
                r#"BSD 3-Clause No Nuclear Warranty"#,
            ),
            (
                r#"BSD-3-Clause-Open-MPI"#,
                r#"BSD 3-Clause Open MPI variant"#,
            ),
            (
                r#"BSD-4-Clause"#,
                r#"BSD 4-Clause "Original" or "Old" License"#,
            ),
            (
                r#"BSD-4-Clause-UC"#,
                r#"BSD-4-Clause (University of California-Specific)"#,
            ),
            (r#"BSD-Protection"#, r#"BSD Protection License"#),
            (r#"BSD-Source-Code"#, r#"BSD Source Code Attribution"#),
            (r#"BSL-1.0"#, r#"Boost Software License 1.0"#),
            (r#"bzip2-1.0.5"#, r#"bzip2 and libbzip2 License v1.0.5"#),
            (r#"bzip2-1.0.6"#, r#"bzip2 and libbzip2 License v1.0.6"#),
            (r#"Caldera"#, r#"Caldera License"#),
            (
                r#"CATOSL-1.1"#,
                r#"Computer Associates Trusted Open Source License 1.1"#,
            ),
            (r#"CC0-1.0"#, r#"Creative Commons Zero v1.0 Universal"#),
            (
                r#"CC-BY-1.0"#,
                r#"Creative Commons Attribution 1.0 Generic"#,
            ),
            (
                r#"CC-BY-2.0"#,
                r#"Creative Commons Attribution 2.0 Generic"#,
            ),
            (
                r#"CC-BY-2.5"#,
                r#"Creative Commons Attribution 2.5 Generic"#,
            ),
            (
                r#"CC-BY-3.0"#,
                r#"Creative Commons Attribution 3.0 Unported"#,
            ),
            (
                r#"CC-BY-4.0"#,
                r#"Creative Commons Attribution 4.0 International"#,
            ),
            (
                r#"CC-BY-NC-1.0"#,
                r#"Creative Commons Attribution Non Commercial 1.0 Generic"#,
            ),
            (
                r#"CC-BY-NC-2.0"#,
                r#"Creative Commons Attribution Non Commercial 2.0 Generic"#,
            ),
            (
                r#"CC-BY-NC-2.5"#,
                r#"Creative Commons Attribution Non Commercial 2.5 Generic"#,
            ),
            (
                r#"CC-BY-NC-3.0"#,
                r#"Creative Commons Attribution Non Commercial 3.0 Unported"#,
            ),
            (
                r#"CC-BY-NC-4.0"#,
                r#"Creative Commons Attribution Non Commercial 4.0 International"#,
            ),
            (
                r#"CC-BY-NC-ND-1.0"#,
                r#"Creative Commons Attribution Non Commercial No Derivatives 1.0 Generic"#,
            ),
            (
                r#"CC-BY-NC-ND-2.0"#,
                r#"Creative Commons Attribution Non Commercial No Derivatives 2.0 Generic"#,
            ),
            (
                r#"CC-BY-NC-ND-2.5"#,
                r#"Creative Commons Attribution Non Commercial No Derivatives 2.5 Generic"#,
            ),
            (
                r#"CC-BY-NC-ND-3.0"#,
                r#"Creative Commons Attribution Non Commercial No Derivatives 3.0 Unported"#,
            ),
            (
                r#"CC-BY-NC-ND-4.0"#,
                r#"Creative Commons Attribution Non Commercial No Derivatives 4.0 International"#,
            ),
            (
                r#"CC-BY-NC-SA-1.0"#,
                r#"Creative Commons Attribution Non Commercial Share Alike 1.0 Generic"#,
            ),
            (
                r#"CC-BY-NC-SA-2.0"#,
                r#"Creative Commons Attribution Non Commercial Share Alike 2.0 Generic"#,
            ),
            (
                r#"CC-BY-NC-SA-2.5"#,
                r#"Creative Commons Attribution Non Commercial Share Alike 2.5 Generic"#,
            ),
            (
                r#"CC-BY-NC-SA-3.0"#,
                r#"Creative Commons Attribution Non Commercial Share Alike 3.0 Unported"#,
            ),
            (
                r#"CC-BY-NC-SA-4.0"#,
                r#"Creative Commons Attribution Non Commercial Share Alike 4.0 International"#,
            ),
            (
                r#"CC-BY-ND-1.0"#,
                r#"Creative Commons Attribution No Derivatives 1.0 Generic"#,
            ),
            (
                r#"CC-BY-ND-2.0"#,
                r#"Creative Commons Attribution No Derivatives 2.0 Generic"#,
            ),
            (
                r#"CC-BY-ND-2.5"#,
                r#"Creative Commons Attribution No Derivatives 2.5 Generic"#,
            ),
            (
                r#"CC-BY-ND-3.0"#,
                r#"Creative Commons Attribution No Derivatives 3.0 Unported"#,
            ),
            (
                r#"CC-BY-ND-4.0"#,
                r#"Creative Commons Attribution No Derivatives 4.0 International"#,
            ),
            (
                r#"CC-BY-SA-1.0"#,
                r#"Creative Commons Attribution Share Alike 1.0 Generic"#,
            ),
            (
                r#"CC-BY-SA-2.0"#,
                r#"Creative Commons Attribution Share Alike 2.0 Generic"#,
            ),
            (
                r#"CC-BY-SA-2.5"#,
                r#"Creative Commons Attribution Share Alike 2.5 Generic"#,
            ),
            (
                r#"CC-BY-SA-3.0"#,
                r#"Creative Commons Attribution Share Alike 3.0 Unported"#,
            ),
            (
                r#"CC-BY-SA-4.0"#,
                r#"Creative Commons Attribution Share Alike 4.0 International"#,
            ),
            (
                r#"CC-PDDC"#,
                r#"Creative Commons Public Domain Dedication and Certification"#,
            ),
            (
                r#"CDDL-1.0"#,
                r#"Common Development and Distribution License 1.0"#,
            ),
            (
                r#"CDDL-1.1"#,
                r#"Common Development and Distribution License 1.1"#,
            ),
            (
                r#"CDLA-Permissive-1.0"#,
                r#"Community Data License Agreement Permissive 1.0"#,
            ),
            (
                r#"CDLA-Sharing-1.0"#,
                r#"Community Data License Agreement Sharing 1.0"#,
            ),
            (
                r#"CECILL-1.0"#,
                r#"CeCILL Free Software License Agreement v1.0"#,
            ),
            (
                r#"CECILL-1.1"#,
                r#"CeCILL Free Software License Agreement v1.1"#,
            ),
            (
                r#"CECILL-2.0"#,
                r#"CeCILL Free Software License Agreement v2.0"#,
            ),
            (
                r#"CECILL-2.1"#,
                r#"CeCILL Free Software License Agreement v2.1"#,
            ),
            (r#"CECILL-B"#, r#"CeCILL-B Free Software License Agreement"#),
            (r#"CECILL-C"#, r#"CeCILL-C Free Software License Agreement"#),
            (r#"CERN-OHL-1.1"#, r#"CERN Open Hardware Licence v1.1"#),
            (r#"CERN-OHL-1.2"#, r#"CERN Open Hardware Licence v1.2"#),
            (r#"ClArtistic"#, r#"Clarified Artistic License"#),
            (r#"CNRI-Jython"#, r#"CNRI Jython License"#),
            (r#"CNRI-Python"#, r#"CNRI Python License"#),
            (
                r#"CNRI-Python-GPL-Compatible"#,
                r#"CNRI Python Open Source GPL Compatible License Agreement"#,
            ),
            (r#"Condor-1.1"#, r#"Condor Public License v1.1"#),
            (r#"copyleft-next-0.3.0"#, r#"copyleft-next 0.3.0"#),
            (r#"copyleft-next-0.3.1"#, r#"copyleft-next 0.3.1"#),
            (r#"CPAL-1.0"#, r#"Common Public Attribution License 1.0"#),
            (r#"CPL-1.0"#, r#"Common Public License 1.0"#),
            (r#"CPOL-1.02"#, r#"Code Project Open License 1.02"#),
            (r#"Crossword"#, r#"Crossword License"#),
            (r#"CrystalStacker"#, r#"CrystalStacker License"#),
            (r#"CUA-OPL-1.0"#, r#"CUA Office Public License v1.0"#),
            (r#"Cube"#, r#"Cube License"#),
            (r#"curl"#, r#"curl License"#),
            (r#"D-FSL-1.0"#, r#"Deutsche Freie Software Lizenz"#),
            (r#"diffmark"#, r#"diffmark license"#),
            (r#"DOC"#, r#"DOC License"#),
            (r#"Dotseqn"#, r#"Dotseqn License"#),
            (r#"DSDP"#, r#"DSDP License"#),
            (r#"dvipdfm"#, r#"dvipdfm License"#),
            (r#"eCos-2.0"#, r#"eCos license version 2.0"#),
            (r#"eGenix"#, r#"eGenix.com Public License 1.1.0"#),
            (r#"ECL-1.0"#, r#"Educational Community License v1.0"#),
            (r#"ECL-2.0"#, r#"Educational Community License v2.0"#),
            (r#"EFL-1.0"#, r#"Eiffel Forum License v1.0"#),
            (r#"EFL-2.0"#, r#"Eiffel Forum License v2.0"#),
            (r#"Entessa"#, r#"Entessa Public License v1.0"#),
            (r#"EPL-1.0"#, r#"Eclipse Public License 1.0"#),
            (r#"EPL-2.0"#, r#"Eclipse Public License 2.0"#),
            (r#"ErlPL-1.1"#, r#"Erlang Public License v1.1"#),
            (r#"etalab-2.0"#, r#"Etalab Open License 2.0"#),
            (r#"EUDatagrid"#, r#"EU DataGrid Software License"#),
            (r#"EUPL-1.0"#, r#"European Union Public License 1.0"#),
            (r#"EUPL-1.1"#, r#"European Union Public License 1.1"#),
            (r#"EUPL-1.2"#, r#"European Union Public License 1.2"#),
            (r#"Eurosym"#, r#"Eurosym License"#),
            (r#"Fair"#, r#"Fair License"#),
            (r#"Frameworx-1.0"#, r#"Frameworx Open License 1.0"#),
            (r#"FreeImage"#, r#"FreeImage Public License v1.0"#),
            (r#"FSFAP"#, r#"FSF All Permissive License"#),
            (r#"FSFUL"#, r#"FSF Unlimited License"#),
            (
                r#"FSFULLR"#,
                r#"FSF Unlimited License (with License Retention)"#,
            ),
            (r#"FTL"#, r#"Freetype Project License"#),
            (r#"gSOAP-1.3b"#, r#"gSOAP Public License v1.3b"#),
            (r#"GFDL-1.1"#, r#"GNU Free Documentation License v1.1"#),
            (
                r#"GFDL-1.1-only"#,
                r#"GNU Free Documentation License v1.1 only"#,
            ),
            (
                r#"GFDL-1.1-or-later"#,
                r#"GNU Free Documentation License v1.1 or later"#,
            ),
            (r#"GFDL-1.2"#, r#"GNU Free Documentation License v1.2"#),
            (
                r#"GFDL-1.2-only"#,
                r#"GNU Free Documentation License v1.2 only"#,
            ),
            (
                r#"GFDL-1.2-or-later"#,
                r#"GNU Free Documentation License v1.2 or later"#,
            ),
            (r#"GFDL-1.3"#, r#"GNU Free Documentation License v1.3"#),
            (
                r#"GFDL-1.3-only"#,
                r#"GNU Free Documentation License v1.3 only"#,
            ),
            (
                r#"GFDL-1.3-or-later"#,
                r#"GNU Free Documentation License v1.3 or later"#,
            ),
            (r#"Giftware"#, r#"Giftware License"#),
            (r#"GL2PS"#, r#"GL2PS License"#),
            (r#"Glide"#, r#"3dfx Glide License"#),
            (r#"Glulxe"#, r#"Glulxe License"#),
            (r#"gnuplot"#, r#"gnuplot License"#),
            (r#"GPL-1.0"#, r#"GNU General Public License v1.0 only"#),
            (r#"GPL-1.0+"#, r#"GNU General Public License v1.0 or later"#),
            (
                r#"GPL-1.0-or-later"#,
                r#"GNU General Public License v1.0 or later"#,
            ),
            (r#"GPL-1.0-only"#, r#"GNU General Public License v1.0 only"#),
            (r#"GPL-2.0"#, r#"GNU General Public License v2.0 only"#),
            (r#"GPL-2.0+"#, r#"GNU General Public License v2.0 or later"#),
            (
                r#"GPL-2.0-with-autoconf-exception"#,
                r#"GNU General Public License v2.0 w/Autoconf exception"#,
            ),
            (
                r#"GPL-2.0-with-bison-exception"#,
                r#"GNU General Public License v2.0 w/Bison exception"#,
            ),
            (
                r#"GPL-2.0-with-classpath-exception"#,
                r#"GNU General Public License v2.0 w/Classpath exception"#,
            ),
            (
                r#"GPL-2.0-with-font-exception"#,
                r#"GNU General Public License v2.0 w/Font exception"#,
            ),
            (
                r#"GPL-2.0-with-GCC-exception"#,
                r#"GNU General Public License v2.0 w/GCC Runtime Library exception"#,
            ),
            (
                r#"GPL-2.0-or-later"#,
                r#"GNU General Public License v2.0 or later"#,
            ),
            (r#"GPL-2.0-only"#, r#"GNU General Public License v2.0 only"#),
            (r#"GPL-3.0"#, r#"GNU General Public License v3.0 only"#),
            (r#"GPL-3.0+"#, r#"GNU General Public License v3.0 or later"#),
            (
                r#"GPL-3.0-with-GCC-exception"#,
                r#"GNU General Public License v3.0 w/GCC Runtime Library exception"#,
            ),
            (
                r#"GPL-3.0-with-autoconf-exception"#,
                r#"GNU General Public License v3.0 w/Autoconf exception"#,
            ),
            (
                r#"GPL-3.0-or-later"#,
                r#"GNU General Public License v3.0 or later"#,
            ),
            (r#"GPL-3.0-only"#, r#"GNU General Public License v3.0 only"#),
            (r#"HaskellReport"#, r#"Haskell Language Report License"#),
            (r#"HPND"#, r#"Historical Permission Notice and Disclaimer"#),
            (
                r#"HPND-sell-variant"#,
                r#"Historical Permission Notice and Disclaimer - sell variant"#,
            ),
            (r#"iMatix"#, r#"iMatix Standard Function Library Agreement"#),
            (
                r#"IBM-pibs"#,
                r#"IBM PowerPC Initialization and Boot Software"#,
            ),
            (r#"ICU"#, r#"ICU License"#),
            (r#"IJG"#, r#"Independent JPEG Group License"#),
            (r#"ImageMagick"#, r#"ImageMagick License"#),
            (r#"Imlib2"#, r#"Imlib2 License"#),
            (r#"Info-ZIP"#, r#"Info-ZIP License"#),
            (r#"Intel"#, r#"Intel Open Source License"#),
            (r#"Intel-ACPI"#, r#"Intel ACPI Software License Agreement"#),
            (r#"Interbase-1.0"#, r#"Interbase Public License v1.0"#),
            (r#"IPA"#, r#"IPA Font License"#),
            (r#"IPL-1.0"#, r#"IBM Public License v1.0"#),
            (r#"ISC"#, r#"ISC License"#),
            (r#"JasPer-2.0"#, r#"JasPer License"#),
            (r#"JPNIC"#, r#"Japan Network Information Center License"#),
            (r#"JSON"#, r#"JSON License"#),
            (r#"LAL-1.2"#, r#"Licence Art Libre 1.2"#),
            (r#"LAL-1.3"#, r#"Licence Art Libre 1.3"#),
            (r#"Latex2e"#, r#"Latex2e License"#),
            (r#"Leptonica"#, r#"Leptonica License"#),
            (
                r#"LGPL-2.0"#,
                r#"GNU Library General Public License v2 only"#,
            ),
            (
                r#"LGPL-2.0+"#,
                r#"GNU Library General Public License v2 or later"#,
            ),
            (
                r#"LGPL-2.1"#,
                r#"GNU Lesser General Public License v2.1 only"#,
            ),
            (
                r#"LGPL-2.1-only"#,
                r#"GNU Lesser General Public License v2.1 only"#,
            ),
            (
                r#"LGPL-2.1-or-later"#,
                r#"GNU Lesser General Public License v2.1 or later"#,
            ),
            (
                r#"LGPL-2.1+"#,
                r#"GNU Library General Public License v2.1 or later"#,
            ),
            (
                r#"LGPL-2.0-or-later"#,
                r#"GNU Library General Public License v2 or later"#,
            ),
            (
                r#"LGPL-2.0-only"#,
                r#"GNU Library General Public License v2 only"#,
            ),
            (
                r#"LGPL-3.0"#,
                r#"GNU Lesser General Public License v3.0 only"#,
            ),
            (
                r#"LGPL-3.0+"#,
                r#"GNU Lesser General Public License v3.0 or later"#,
            ),
            (
                r#"LGPL-3.0-or-later"#,
                r#"GNU Lesser General Public License v3.0 or later"#,
            ),
            (
                r#"LGPL-3.0-only"#,
                r#"GNU Lesser General Public License v3.0 only"#,
            ),
            (
                r#"LGPLLR"#,
                r#"Lesser General Public License For Linguistic Resources"#,
            ),
            (
                r#"LiLiQ-P-1.1"#,
                r#"Licence Libre du Québec – Permissive version 1.1"#,
            ),
            (
                r#"LiLiQ-R-1.1"#,
                r#"Licence Libre du Québec – Réciprocité version 1.1"#,
            ),
            (
                r#"LiLiQ-Rplus-1.1"#,
                r#"Licence Libre du Québec – Réciprocité forte version 1.1"#,
            ),
            (r#"Libpng"#, r#"libpng License"#),
            (r#"libpng-2.0"#, r#"PNG Reference Library version 2"#),
            (r#"libtiff"#, r#"libtiff License"#),
            (
                r#"Linux-OpenIB"#,
                r#"Linux Kernel Variant of OpenIB.org license"#,
            ),
            (r#"LPL-1.0"#, r#"Lucent Public License Version 1.0"#),
            (r#"LPL-1.02"#, r#"Lucent Public License v1.02"#),
            (r#"LPPL-1.0"#, r#"LaTeX Project Public License v1.0"#),
            (r#"LPPL-1.1"#, r#"LaTeX Project Public License v1.1"#),
            (r#"LPPL-1.2"#, r#"LaTeX Project Public License v1.2"#),
            (r#"LPPL-1.3a"#, r#"LaTeX Project Public License v1.3a"#),
            (r#"LPPL-1.3c"#, r#"LaTeX Project Public License v1.3c"#),
            (r#"MakeIndex"#, r#"MakeIndex License"#),
            (r#"MirOS"#, r#"The MirOS Licence"#),
            (r#"MIT"#, r#"MIT License"#),
            (r#"MIT-0"#, r#"MIT No Attribution"#),
            (r#"MIT-advertising"#, r#"Enlightenment License (e16)"#),
            (r#"MIT-CMU"#, r#"CMU License"#),
            (r#"MIT-enna"#, r#"enna License"#),
            (r#"MIT-feh"#, r#"feh License"#),
            (r#"MITNFA"#, r#"MIT +no-false-attribs license"#),
            (r#"Motosoto"#, r#"Motosoto License"#),
            (r#"mpich2"#, r#"mpich2 License"#),
            (r#"MPL-1.0"#, r#"Mozilla Public License 1.0"#),
            (r#"MPL-1.1"#, r#"Mozilla Public License 1.1"#),
            (r#"MPL-2.0"#, r#"Mozilla Public License 2.0"#),
            (
                r#"MPL-2.0-no-copyleft-exception"#,
                r#"Mozilla Public License 2.0 (no copyleft exception)"#,
            ),
            (r#"MS-PL"#, r#"Microsoft Public License"#),
            (r#"MS-RL"#, r#"Microsoft Reciprocal License"#),
            (r#"MTLL"#, r#"Matrix Template Library License"#),
            (
                r#"MulanPSL-1.0"#,
                r#"Mulan Permissive Software License, Version 1"#,
            ),
            (r#"Multics"#, r#"Multics License"#),
            (r#"Mup"#, r#"Mup License"#),
            (r#"NASA-1.3"#, r#"NASA Open Source Agreement 1.3"#),
            (r#"Naumen"#, r#"Naumen Public License"#),
            (r#"NBPL-1.0"#, r#"Net Boolean Public License v1"#),
            (
                r#"NCSA"#,
                r#"University of Illinois/NCSA Open Source License"#,
            ),
            (r#"NetCDF"#, r#"NetCDF license"#),
            (r#"Net-SNMP"#, r#"Net-SNMP License"#),
            (r#"Newsletr"#, r#"Newsletr License"#),
            (r#"NGPL"#, r#"Nethack General Public License"#),
            (
                r#"NLOD-1.0"#,
                r#"Norwegian Licence for Open Government Data"#,
            ),
            (r#"NLPL"#, r#"No Limit Public License"#),
            (r#"Nokia"#, r#"Nokia Open Source License"#),
            (r#"NOSL"#, r#"Netizen Open Source License"#),
            (r#"Noweb"#, r#"Noweb License"#),
            (r#"NPL-1.0"#, r#"Netscape Public License v1.0"#),
            (r#"NPL-1.1"#, r#"Netscape Public License v1.1"#),
            (r#"NPOSL-3.0"#, r#"Non-Profit Open Software License 3.0"#),
            (r#"NRL"#, r#"NRL License"#),
            (r#"NTP"#, r#"NTP License"#),
            (r#"Nunit"#, r#"Nunit License"#),
            (r#"OCCT-PL"#, r#"Open CASCADE Technology Public License"#),
            (r#"OCLC-2.0"#, r#"OCLC Research Public License 2.0"#),
            (r#"ODbL-1.0"#, r#"ODC Open Database License v1.0"#),
            (
                r#"ODC-By-1.0"#,
                r#"Open Data Commons Attribution License v1.0"#,
            ),
            (r#"OFL-1.0"#, r#"SIL Open Font License 1.0"#),
            (r#"OFL-1.1"#, r#"SIL Open Font License 1.1"#),
            (r#"OGL-Canada-2.0"#, r#"Open Government Licence - Canada"#),
            (r#"OGL-UK-1.0"#, r#"Open Government Licence v1.0"#),
            (r#"OGL-UK-2.0"#, r#"Open Government Licence v2.0"#),
            (r#"OGL-UK-3.0"#, r#"Open Government Licence v3.0"#),
            (r#"OGTSL"#, r#"Open Group Test Suite License"#),
            (r#"OLDAP-1.1"#, r#"Open LDAP Public License v1.1"#),
            (r#"OLDAP-1.2"#, r#"Open LDAP Public License v1.2"#),
            (r#"OLDAP-1.3"#, r#"Open LDAP Public License v1.3"#),
            (r#"OLDAP-1.4"#, r#"Open LDAP Public License v1.4"#),
            (
                r#"OLDAP-2.0"#,
                r#"Open LDAP Public License v2.0 (or possibly 2.0A and 2.0B)"#,
            ),
            (r#"OLDAP-2.0.1"#, r#"Open LDAP Public License v2.0.1"#),
            (r#"OLDAP-2.1"#, r#"Open LDAP Public License v2.1"#),
            (r#"OLDAP-2.2"#, r#"Open LDAP Public License v2.2"#),
            (r#"OLDAP-2.2.1"#, r#"Open LDAP Public License v2.2.1"#),
            (r#"OLDAP-2.2.2"#, r#"Open LDAP Public License 2.2.2"#),
            (r#"OLDAP-2.3"#, r#"Open LDAP Public License v2.3"#),
            (r#"OLDAP-2.4"#, r#"Open LDAP Public License v2.4"#),
            (r#"OLDAP-2.5"#, r#"Open LDAP Public License v2.5"#),
            (r#"OLDAP-2.6"#, r#"Open LDAP Public License v2.6"#),
            (r#"OLDAP-2.7"#, r#"Open LDAP Public License v2.7"#),
            (r#"OLDAP-2.8"#, r#"Open LDAP Public License v2.8"#),
            (r#"OML"#, r#"Open Market License"#),
            (r#"OpenSSL"#, r#"OpenSSL License"#),
            (r#"OPL-1.0"#, r#"Open Public License v1.0"#),
            (r#"OSET-PL-2.1"#, r#"OSET Public License version 2.1"#),
            (r#"OSL-1.0"#, r#"Open Software License 1.0"#),
            (r#"OSL-1.1"#, r#"Open Software License 1.1"#),
            (r#"OSL-2.0"#, r#"Open Software License 2.0"#),
            (r#"OSL-2.1"#, r#"Open Software License 2.1"#),
            (r#"OSL-3.0"#, r#"Open Software License 3.0"#),
            (r#"Parity-6.0.0"#, r#"The Parity Public License 6.0.0"#),
            (
                r#"PDDL-1.0"#,
                r#"ODC Public Domain Dedication & License 1.0"#,
            ),
            (r#"PHP-3.0"#, r#"PHP License v3.0"#),
            (r#"PHP-3.01"#, r#"PHP License v3.01"#),
            (r#"Plexus"#, r#"Plexus Classworlds License"#),
            (r#"PostgreSQL"#, r#"PostgreSQL License"#),
            (r#"psfrag"#, r#"psfrag License"#),
            (r#"psutils"#, r#"psutils License"#),
            (r#"Python-2.0"#, r#"Python License 2.0"#),
            (r#"Qhull"#, r#"Qhull License"#),
            (r#"QPL-1.0"#, r#"Q Public License 1.0"#),
            (r#"Rdisc"#, r#"Rdisc License"#),
            (r#"RHeCos-1.1"#, r#"Red Hat eCos Public License v1.1"#),
            (r#"RPL-1.1"#, r#"Reciprocal Public License 1.1"#),
            (r#"RPL-1.5"#, r#"Reciprocal Public License 1.5"#),
            (r#"RPSL-1.0"#, r#"RealNetworks Public Source License v1.0"#),
            (r#"RSA-MD"#, r#"RSA Message-Digest License "#),
            (r#"RSCPL"#, r#"Ricoh Source Code Public License"#),
            (r#"Ruby"#, r#"Ruby License"#),
            (r#"SAX-PD"#, r#"Sax Public Domain Notice"#),
            (r#"Saxpath"#, r#"Saxpath License"#),
            (r#"SCEA"#, r#"SCEA Shared Source License"#),
            (r#"Sendmail"#, r#"Sendmail License"#),
            (r#"Sendmail-8.23"#, r#"Sendmail License 8.23"#),
            (r#"SGI-B-1.0"#, r#"SGI Free Software License B v1.0"#),
            (r#"SGI-B-1.1"#, r#"SGI Free Software License B v1.1"#),
            (r#"SGI-B-2.0"#, r#"SGI Free Software License B v2.0"#),
            (r#"SHL-0.5"#, r#"Solderpad Hardware License v0.5"#),
            (r#"SHL-0.51"#, r#"Solderpad Hardware License, Version 0.51"#),
            (r#"SimPL-2.0"#, r#"Simple Public License 2.0"#),
            (r#"SISSL"#, r#"Sun Industry Standards Source License v1.1"#),
            (
                r#"SISSL-1.2"#,
                r#"Sun Industry Standards Source License v1.2"#,
            ),
            (r#"Sleepycat"#, r#"Sleepycat License"#),
            (r#"SMLNJ"#, r#"Standard ML of New Jersey License"#),
            (r#"SMPPL"#, r#"Secure Messaging Protocol Public License"#),
            (r#"SNIA"#, r#"SNIA Public License 1.1"#),
            (r#"Spencer-86"#, r#"Spencer License 86"#),
            (r#"Spencer-94"#, r#"Spencer License 94"#),
            (r#"Spencer-99"#, r#"Spencer License 99"#),
            (r#"SPL-1.0"#, r#"Sun Public License v1.0"#),
            (r#"SSH-OpenSSH"#, r#"SSH OpenSSH license"#),
            (r#"SSH-short"#, r#"SSH short notice"#),
            (r#"SSPL-1.0"#, r#"Server Side Public License, v 1"#),
            (r#"StandardML-NJ"#, r#"Standard ML of New Jersey License"#),
            (r#"SugarCRM-1.1.3"#, r#"SugarCRM Public License v1.1.3"#),
            (
                r#"SWL"#,
                r#"Scheme Widget Library (SWL) Software License Agreement"#,
            ),
            (r#"TAPR-OHL-1.0"#, r#"TAPR Open Hardware License v1.0"#),
            (r#"TCL"#, r#"TCL/TK License"#),
            (r#"TCP-wrappers"#, r#"TCP Wrappers License"#),
            (r#"TMate"#, r#"TMate Open Source License"#),
            (r#"TORQUE-1.1"#, r#"TORQUE v2.5+ Software License v1.1"#),
            (r#"TOSL"#, r#"Trusster Open Source License"#),
            (
                r#"TU-Berlin-1.0"#,
                r#"Technische Universitaet Berlin License 1.0"#,
            ),
            (
                r#"TU-Berlin-2.0"#,
                r#"Technische Universitaet Berlin License 2.0"#,
            ),
            (r#"UCL-1.0"#, r#"Upstream Compatibility License v1.0"#),
            (
                r#"Unicode-DFS-2015"#,
                r#"Unicode License Agreement - Data Files and Software (2015)"#,
            ),
            (
                r#"Unicode-DFS-2016"#,
                r#"Unicode License Agreement - Data Files and Software (2016)"#,
            ),
            (r#"Unicode-TOU"#, r#"Unicode Terms of Use"#),
            (r#"Unlicense"#, r#"The Unlicense"#),
            (r#"UPL-1.0"#, r#"Universal Permissive License v1.0"#),
            (r#"Vim"#, r#"Vim License"#),
            (r#"VOSTROM"#, r#"VOSTROM Public License for Open Source"#),
            (r#"VSL-1.0"#, r#"Vovida Software License v1.0"#),
            (r#"W3C"#, r#"W3C Software Notice and License (2002-12-31)"#),
            (
                r#"W3C-19980720"#,
                r#"W3C Software Notice and License (1998-07-20)"#,
            ),
            (
                r#"W3C-20150513"#,
                r#"W3C Software Notice and Document License (2015-05-13)"#,
            ),
            (r#"Watcom-1.0"#, r#"Sybase Open Watcom Public License 1.0"#),
            (r#"Wsuipa"#, r#"Wsuipa License"#),
            (r#"WTFPL"#, r#"Do What The F*ck You Want To Public License"#),
            (r#"wxWindows"#, r#"wxWindows Library License"#),
            (r#"X11"#, r#"X11 License"#),
            (r#"Xerox"#, r#"Xerox License"#),
            (r#"XFree86-1.1"#, r#"XFree86 License 1.1"#),
            (r#"xinetd"#, r#"xinetd License"#),
            (r#"Xnet"#, r#"X.Net License"#),
            (r#"xpp"#, r#"XPP License"#),
            (r#"XSkat"#, r#"XSkat License"#),
            (r#"YPL-1.0"#, r#"Yahoo! Public License v1.0"#),
            (r#"YPL-1.1"#, r#"Yahoo! Public License v1.1"#),
            (r#"Zed"#, r#"Zed License"#),
            (r#"Zend-2.0"#, r#"Zend License v2.0"#),
            (r#"Zimbra-1.3"#, r#"Zimbra Public License v1.3"#),
            (r#"Zimbra-1.4"#, r#"Zimbra Public License v1.4"#),
            (r#"Zlib"#, r#"zlib License"#),
            (
                r#"zlib-acknowledgement"#,
                r#"zlib/libpng License with Acknowledgement"#,
            ),
            (r#"ZPL-1.1"#, r#"Zope Public License 1.1"#),
            (r#"ZPL-2.0"#, r#"Zope Public License 2.0"#),
            (r#"ZPL-2.1"#, r#"Zope Public License 2.1"#),
        ],
        bool_properties: [
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new(),
            BoolProperties::new(),
            BoolProperties::new().set_osi_approved(true),
            BoolProperties::new().set_osi_approved(true),
        ],
    };

    // Creates static a hash map if `phf` is enabled, else resorts to a good ol'
    // `match` statement :D
    pub(crate) fn _from_id(id: &str) -> Option<Self> {
        #[cfg(feature = "phf")]
        {
            static ID_TO_LICENSE: phf::Map<&str, SpdxLicense> = phf::phf_map! {
                "AAL" => SpdxLicense::Aal,
            "Abstyles" => SpdxLicense::Abstyles,
            "Adobe-2006" => SpdxLicense::Adobe2006,
            "Adobe-Glyph" => SpdxLicense::AdobeGlyph,
            "ADSL" => SpdxLicense::Adsl,
            "AFL-1.1" => SpdxLicense::Afl1_1,
            "AFL-1.2" => SpdxLicense::Afl1_2,
            "AFL-2.0" => SpdxLicense::Afl2,
            "AFL-2.1" => SpdxLicense::Afl2_1,
            "AFL-3.0" => SpdxLicense::Afl3,
            "Afmparse" => SpdxLicense::Afmparse,
            "AGPL-1.0" => SpdxLicense::Agpl1,
            "AGPL-1.0-or-later" => SpdxLicense::Agpl1r,
            "AGPL-1.0-only" => SpdxLicense::Agpl1y,
            "AGPL-3.0" => SpdxLicense::Agpl3,
            "AGPL-3.0-or-later" => SpdxLicense::Agpl3r,
            "AGPL-3.0-only" => SpdxLicense::Agpl3y,
            "Aladdin" => SpdxLicense::Aladdin,
            "AMDPLPA" => SpdxLicense::Amdplpa,
            "AML" => SpdxLicense::Aml,
            "AMPAS" => SpdxLicense::Ampas,
            "ANTLR-PD" => SpdxLicense::AntlrPd,
            "Apache-1.0" => SpdxLicense::Apache1,
            "Apache-1.1" => SpdxLicense::Apache1_1,
            "Apache-2.0" => SpdxLicense::Apache2,
            "APAFML" => SpdxLicense::Apafml,
            "APL-1.0" => SpdxLicense::Apl1,
            "APSL-1.0" => SpdxLicense::Apsl1,
            "APSL-1.1" => SpdxLicense::Apsl1_1,
            "APSL-1.2" => SpdxLicense::Apsl1_2,
            "APSL-2.0" => SpdxLicense::Apsl2,
            "Artistic-1.0" => SpdxLicense::Artistic1,
            "Artistic-1.0-Perl" => SpdxLicense::Artistic1l,
            "Artistic-1.0-cl8" => SpdxLicense::Artistic1l8,
            "Artistic-2.0" => SpdxLicense::Artistic2,
            "Bahyph" => SpdxLicense::Bahyph,
            "Barr" => SpdxLicense::Barr,
            "Beerware" => SpdxLicense::Beerware,
            "BitTorrent-1.0" => SpdxLicense::BitTorrent1,
            "BitTorrent-1.1" => SpdxLicense::BitTorrent1_1,
            "blessing" => SpdxLicense::Blessing,
            "BlueOak-1.0.0" => SpdxLicense::BlueOak1,
            "Borceux" => SpdxLicense::Borceux,
            "0BSD" => SpdxLicense::Bsd0,
            "BSD-1-Clause" => SpdxLicense::Bsd1Clause,
            "BSD-2-Clause" => SpdxLicense::Bsd2Clause,
            "BSD-2-Clause-FreeBSD" => SpdxLicense::Bsd2ClauseFreeBsd,
            "BSD-2-Clause-NetBSD" => SpdxLicense::Bsd2ClauseNetBsd,
            "BSD-2-Clause-Patent" => SpdxLicense::Bsd2ClausePatent,
            "BSD-3-Clause" => SpdxLicense::Bsd3Clause,
            "BSD-3-Clause-Attribution" => SpdxLicense::Bsd3ClauseAttribution,
            "BSD-3-Clause-Clear" => SpdxLicense::Bsd3ClauseClear,
            "BSD-3-Clause-LBNL" => SpdxLicense::Bsd3ClauseLbnl,
            "BSD-3-Clause-No-Nuclear-License" => SpdxLicense::Bsd3ClauseNoNuclearLicense,
            "BSD-3-Clause-No-Nuclear-License-2014" => SpdxLicense::Bsd3ClauseNoNuclearLicense2014,
            "BSD-3-Clause-No-Nuclear-Warranty" => SpdxLicense::Bsd3ClauseNoNuclearWarranty,
            "BSD-3-Clause-Open-MPI" => SpdxLicense::Bsd3ClauseOpenMpi,
            "BSD-4-Clause" => SpdxLicense::Bsd4Clause,
            "BSD-4-Clause-UC" => SpdxLicense::Bsd4ClauseUc,
            "BSD-Protection" => SpdxLicense::BsdProtection,
            "BSD-Source-Code" => SpdxLicense::BsdSourceCode,
            "BSL-1.0" => SpdxLicense::Bsl1,
            "bzip2-1.0.5" => SpdxLicense::Bzip2_1_5,
            "bzip2-1.0.6" => SpdxLicense::Bzip2_1_6,
            "Caldera" => SpdxLicense::Caldera,
            "CATOSL-1.1" => SpdxLicense::Catosl1_1,
            "CC0-1.0" => SpdxLicense::Cc0_1,
            "CC-BY-1.0" => SpdxLicense::CcBy1,
            "CC-BY-2.0" => SpdxLicense::CcBy2,
            "CC-BY-2.5" => SpdxLicense::CcBy2_5,
            "CC-BY-3.0" => SpdxLicense::CcBy3,
            "CC-BY-4.0" => SpdxLicense::CcBy4,
            "CC-BY-NC-1.0" => SpdxLicense::CcByNc1,
            "CC-BY-NC-2.0" => SpdxLicense::CcByNc2,
            "CC-BY-NC-2.5" => SpdxLicense::CcByNc2_5,
            "CC-BY-NC-3.0" => SpdxLicense::CcByNc3,
            "CC-BY-NC-4.0" => SpdxLicense::CcByNc4,
            "CC-BY-NC-ND-1.0" => SpdxLicense::CcByNcNd1,
            "CC-BY-NC-ND-2.0" => SpdxLicense::CcByNcNd2,
            "CC-BY-NC-ND-2.5" => SpdxLicense::CcByNcNd2_5,
            "CC-BY-NC-ND-3.0" => SpdxLicense::CcByNcNd3,
            "CC-BY-NC-ND-4.0" => SpdxLicense::CcByNcNd4,
            "CC-BY-NC-SA-1.0" => SpdxLicense::CcByNcSa1,
            "CC-BY-NC-SA-2.0" => SpdxLicense::CcByNcSa2,
            "CC-BY-NC-SA-2.5" => SpdxLicense::CcByNcSa2_5,
            "CC-BY-NC-SA-3.0" => SpdxLicense::CcByNcSa3,
            "CC-BY-NC-SA-4.0" => SpdxLicense::CcByNcSa4,
            "CC-BY-ND-1.0" => SpdxLicense::CcByNd1,
            "CC-BY-ND-2.0" => SpdxLicense::CcByNd2,
            "CC-BY-ND-2.5" => SpdxLicense::CcByNd2_5,
            "CC-BY-ND-3.0" => SpdxLicense::CcByNd3,
            "CC-BY-ND-4.0" => SpdxLicense::CcByNd4,
            "CC-BY-SA-1.0" => SpdxLicense::CcBySa1,
            "CC-BY-SA-2.0" => SpdxLicense::CcBySa2,
            "CC-BY-SA-2.5" => SpdxLicense::CcBySa2_5,
            "CC-BY-SA-3.0" => SpdxLicense::CcBySa3,
            "CC-BY-SA-4.0" => SpdxLicense::CcBySa4,
            "CC-PDDC" => SpdxLicense::CcPddc,
            "CDDL-1.0" => SpdxLicense::Cddl1,
            "CDDL-1.1" => SpdxLicense::Cddl1_1,
            "CDLA-Permissive-1.0" => SpdxLicense::CdlaPermissive1,
            "CDLA-Sharing-1.0" => SpdxLicense::CdlaSharing1,
            "CECILL-1.0" => SpdxLicense::Cecill1,
            "CECILL-1.1" => SpdxLicense::Cecill1_1,
            "CECILL-2.0" => SpdxLicense::Cecill2,
            "CECILL-2.1" => SpdxLicense::Cecill2_1,
            "CECILL-B" => SpdxLicense::CecillB,
            "CECILL-C" => SpdxLicense::CecillC,
            "CERN-OHL-1.1" => SpdxLicense::CernOhl1_1,
            "CERN-OHL-1.2" => SpdxLicense::CernOhl1_2,
            "ClArtistic" => SpdxLicense::ClArtistic,
            "CNRI-Jython" => SpdxLicense::CnriJython,
            "CNRI-Python" => SpdxLicense::CnriPython,
            "CNRI-Python-GPL-Compatible" => SpdxLicense::CnriPythonGplCompatible,
            "Condor-1.1" => SpdxLicense::Condor1_1,
            "copyleft-next-0.3.0" => SpdxLicense::CopyleftNext0_3,
            "copyleft-next-0.3.1" => SpdxLicense::CopyleftNext0_3_1,
            "CPAL-1.0" => SpdxLicense::Cpal1,
            "CPL-1.0" => SpdxLicense::Cpl1,
            "CPOL-1.02" => SpdxLicense::Cpol12,
            "Crossword" => SpdxLicense::Crossword,
            "CrystalStacker" => SpdxLicense::CrystalStacker,
            "CUA-OPL-1.0" => SpdxLicense::CuaOpl1,
            "Cube" => SpdxLicense::Cube,
            "curl" => SpdxLicense::Curl,
            "D-FSL-1.0" => SpdxLicense::DFsl1,
            "diffmark" => SpdxLicense::Diffmark,
            "DOC" => SpdxLicense::Doc,
            "Dotseqn" => SpdxLicense::Dotseqn,
            "DSDP" => SpdxLicense::Dsdp,
            "dvipdfm" => SpdxLicense::Dvipdfm,
            "eCos-2.0" => SpdxLicense::ECos2,
            "eGenix" => SpdxLicense::EGenix,
            "ECL-1.0" => SpdxLicense::Ecl1,
            "ECL-2.0" => SpdxLicense::Ecl2,
            "EFL-1.0" => SpdxLicense::Efl1,
            "EFL-2.0" => SpdxLicense::Efl2,
            "Entessa" => SpdxLicense::Entessa,
            "EPL-1.0" => SpdxLicense::Epl1,
            "EPL-2.0" => SpdxLicense::Epl2,
            "ErlPL-1.1" => SpdxLicense::ErlPl1_1,
            "etalab-2.0" => SpdxLicense::Etalab2,
            "EUDatagrid" => SpdxLicense::Eudatagrid,
            "EUPL-1.0" => SpdxLicense::Eupl1,
            "EUPL-1.1" => SpdxLicense::Eupl1_1,
            "EUPL-1.2" => SpdxLicense::Eupl1_2,
            "Eurosym" => SpdxLicense::Eurosym,
            "Fair" => SpdxLicense::Fair,
            "Frameworx-1.0" => SpdxLicense::Frameworx1,
            "FreeImage" => SpdxLicense::FreeImage,
            "FSFAP" => SpdxLicense::Fsfap,
            "FSFUL" => SpdxLicense::Fsful,
            "FSFULLR" => SpdxLicense::Fsfullr,
            "FTL" => SpdxLicense::Ftl,
            "gSOAP-1.3b" => SpdxLicense::GSoap1_3b,
            "GFDL-1.1" => SpdxLicense::Gfdl1_1,
            "GFDL-1.1-only" => SpdxLicense::Gfdl1_1Only,
            "GFDL-1.1-or-later" => SpdxLicense::Gfdl1_1OrLater,
            "GFDL-1.2" => SpdxLicense::Gfdl1_2,
            "GFDL-1.2-only" => SpdxLicense::Gfdl1_2Only,
            "GFDL-1.2-or-later" => SpdxLicense::Gfdl1_2OrLater,
            "GFDL-1.3" => SpdxLicense::Gfdl1_3,
            "GFDL-1.3-only" => SpdxLicense::Gfdl1_3Only,
            "GFDL-1.3-or-later" => SpdxLicense::Gfdl1_3OrLater,
            "Giftware" => SpdxLicense::Giftware,
            "GL2PS" => SpdxLicense::Gl2Ps,
            "Glide" => SpdxLicense::Glide,
            "Glulxe" => SpdxLicense::Glulxe,
            "gnuplot" => SpdxLicense::Gnuplot,
            "GPL-1.0" => SpdxLicense::Gpl1,
            "GPL-1.0+" => SpdxLicense::Gpl1Plus,
            "GPL-1.0-or-later" => SpdxLicense::Gpl1r,
            "GPL-1.0-only" => SpdxLicense::Gpl1y,
            "GPL-2.0" => SpdxLicense::Gpl2,
            "GPL-2.0+" => SpdxLicense::Gpl2Plus,
            "GPL-2.0-with-autoconf-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-bison-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-classpath-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-font-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-GCC-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-or-later" => SpdxLicense::Gpl2r,
            "GPL-2.0-only" => SpdxLicense::Gpl2y,
            "GPL-3.0" => SpdxLicense::Gpl3,
            "GPL-3.0+" => SpdxLicense::Gpl3Plus,
            "GPL-3.0-with-GCC-exception" => SpdxLicense::Gpl3n,
            "GPL-3.0-with-autoconf-exception" => SpdxLicense::Gpl3n,
            "GPL-3.0-or-later" => SpdxLicense::Gpl3r,
            "GPL-3.0-only" => SpdxLicense::Gpl3y,
            "HaskellReport" => SpdxLicense::HaskellReport,
            "HPND" => SpdxLicense::Hpnd,
            "HPND-sell-variant" => SpdxLicense::HpndSellVariant,
            "iMatix" => SpdxLicense::IMatix,
            "IBM-pibs" => SpdxLicense::IbmPibs,
            "ICU" => SpdxLicense::Icu,
            "IJG" => SpdxLicense::Ijg,
            "ImageMagick" => SpdxLicense::ImageMagick,
            "Imlib2" => SpdxLicense::Imlib2,
            "Info-ZIP" => SpdxLicense::InfoZip,
            "Intel" => SpdxLicense::Intel,
            "Intel-ACPI" => SpdxLicense::IntelAcpi,
            "Interbase-1.0" => SpdxLicense::Interbase1,
            "IPA" => SpdxLicense::Ipa,
            "IPL-1.0" => SpdxLicense::Ipl1,
            "ISC" => SpdxLicense::Isc,
            "JasPer-2.0" => SpdxLicense::JasPer2,
            "JPNIC" => SpdxLicense::Jpnic,
            "JSON" => SpdxLicense::Json,
            "LAL-1.2" => SpdxLicense::Lal1_2,
            "LAL-1.3" => SpdxLicense::Lal1_3,
            "Latex2e" => SpdxLicense::Latex2e,
            "Leptonica" => SpdxLicense::Leptonica,
            "LGPL-2.0" => SpdxLicense::Lgpl2,
            "LGPL-2.0+" => SpdxLicense::Lgpl2Plus,
            "LGPL-2.1" => SpdxLicense::Lgpl2_1,
            "LGPL-2.1-only" => SpdxLicense::Lgpl2_1Only,
            "LGPL-2.1-or-later" => SpdxLicense::Lgpl2_1OrLater,
            "LGPL-2.1+" => SpdxLicense::Lgpl2_1Plus,
            "LGPL-2.0-or-later" => SpdxLicense::Lgpl2r,
            "LGPL-2.0-only" => SpdxLicense::Lgpl2y,
            "LGPL-3.0" => SpdxLicense::Lgpl3,
            "LGPL-3.0+" => SpdxLicense::Lgpl3Plus,
            "LGPL-3.0-or-later" => SpdxLicense::Lgpl3r,
            "LGPL-3.0-only" => SpdxLicense::Lgpl3y,
            "LGPLLR" => SpdxLicense::Lgpllr,
            "LiLiQ-P-1.1" => SpdxLicense::LiLiQP1_1,
            "LiLiQ-R-1.1" => SpdxLicense::LiLiQR1_1,
            "LiLiQ-Rplus-1.1" => SpdxLicense::LiLiQRplus1_1,
            "Libpng" => SpdxLicense::Libpng,
            "libpng-2.0" => SpdxLicense::Libpng2,
            "libtiff" => SpdxLicense::Libtiff,
            "Linux-OpenIB" => SpdxLicense::LinuxOpenIb,
            "LPL-1.0" => SpdxLicense::Lpl1,
            "LPL-1.02" => SpdxLicense::Lpl12,
            "LPPL-1.0" => SpdxLicense::Lppl1,
            "LPPL-1.1" => SpdxLicense::Lppl1_1,
            "LPPL-1.2" => SpdxLicense::Lppl1_2,
            "LPPL-1.3a" => SpdxLicense::Lppl1_3a,
            "LPPL-1.3c" => SpdxLicense::Lppl1_3c,
            "MakeIndex" => SpdxLicense::MakeIndex,
            "MirOS" => SpdxLicense::MirOs,
            "MIT" => SpdxLicense::Mit,
            "MIT-0" => SpdxLicense::Mit0,
            "MIT-advertising" => SpdxLicense::MitAdvertising,
            "MIT-CMU" => SpdxLicense::MitCmu,
            "MIT-enna" => SpdxLicense::MitEnna,
            "MIT-feh" => SpdxLicense::MitFeh,
            "MITNFA" => SpdxLicense::Mitnfa,
            "Motosoto" => SpdxLicense::Motosoto,
            "mpich2" => SpdxLicense::Mpich2,
            "MPL-1.0" => SpdxLicense::Mpl1,
            "MPL-1.1" => SpdxLicense::Mpl1_1,
            "MPL-2.0" => SpdxLicense::Mpl2,
            "MPL-2.0-no-copyleft-exception" => SpdxLicense::Mpl2n,
            "MS-PL" => SpdxLicense::MsPl,
            "MS-RL" => SpdxLicense::MsRl,
            "MTLL" => SpdxLicense::Mtll,
            "MulanPSL-1.0" => SpdxLicense::MulanPsl1,
            "Multics" => SpdxLicense::Multics,
            "Mup" => SpdxLicense::Mup,
            "NASA-1.3" => SpdxLicense::Nasa1_3,
            "Naumen" => SpdxLicense::Naumen,
            "NBPL-1.0" => SpdxLicense::Nbpl1,
            "NCSA" => SpdxLicense::Ncsa,
            "NetCDF" => SpdxLicense::NetCdf,
            "Net-SNMP" => SpdxLicense::NetSnmp,
            "Newsletr" => SpdxLicense::Newsletr,
            "NGPL" => SpdxLicense::Ngpl,
            "NLOD-1.0" => SpdxLicense::Nlod1,
            "NLPL" => SpdxLicense::Nlpl,
            "Nokia" => SpdxLicense::Nokia,
            "NOSL" => SpdxLicense::Nosl,
            "Noweb" => SpdxLicense::Noweb,
            "NPL-1.0" => SpdxLicense::Npl1,
            "NPL-1.1" => SpdxLicense::Npl1_1,
            "NPOSL-3.0" => SpdxLicense::Nposl3,
            "NRL" => SpdxLicense::Nrl,
            "NTP" => SpdxLicense::Ntp,
            "Nunit" => SpdxLicense::Nunit,
            "OCCT-PL" => SpdxLicense::OcctPl,
            "OCLC-2.0" => SpdxLicense::Oclc2,
            "ODbL-1.0" => SpdxLicense::OdbL1,
            "ODC-By-1.0" => SpdxLicense::OdcBy1,
            "OFL-1.0" => SpdxLicense::Ofl1,
            "OFL-1.1" => SpdxLicense::Ofl1_1,
            "OGL-Canada-2.0" => SpdxLicense::OglCanada2,
            "OGL-UK-1.0" => SpdxLicense::OglUk1,
            "OGL-UK-2.0" => SpdxLicense::OglUk2,
            "OGL-UK-3.0" => SpdxLicense::OglUk3,
            "OGTSL" => SpdxLicense::Ogtsl,
            "OLDAP-1.1" => SpdxLicense::Oldap1_1,
            "OLDAP-1.2" => SpdxLicense::Oldap1_2,
            "OLDAP-1.3" => SpdxLicense::Oldap1_3,
            "OLDAP-1.4" => SpdxLicense::Oldap1_4,
            "OLDAP-2.0" => SpdxLicense::Oldap2,
            "OLDAP-2.0.1" => SpdxLicense::Oldap2_1,
            "OLDAP-2.1" => SpdxLicense::Oldap2_1,
            "OLDAP-2.2" => SpdxLicense::Oldap2_2,
            "OLDAP-2.2.1" => SpdxLicense::Oldap2_2_1,
            "OLDAP-2.2.2" => SpdxLicense::Oldap2_2_2,
            "OLDAP-2.3" => SpdxLicense::Oldap2_3,
            "OLDAP-2.4" => SpdxLicense::Oldap2_4,
            "OLDAP-2.5" => SpdxLicense::Oldap2_5,
            "OLDAP-2.6" => SpdxLicense::Oldap2_6,
            "OLDAP-2.7" => SpdxLicense::Oldap2_7,
            "OLDAP-2.8" => SpdxLicense::Oldap2_8,
            "OML" => SpdxLicense::Oml,
            "OpenSSL" => SpdxLicense::OpenSsl,
            "OPL-1.0" => SpdxLicense::Opl1,
            "OSET-PL-2.1" => SpdxLicense::OsetPl2_1,
            "OSL-1.0" => SpdxLicense::Osl1,
            "OSL-1.1" => SpdxLicense::Osl1_1,
            "OSL-2.0" => SpdxLicense::Osl2,
            "OSL-2.1" => SpdxLicense::Osl2_1,
            "OSL-3.0" => SpdxLicense::Osl3,
            "Parity-6.0.0" => SpdxLicense::Parity6,
            "PDDL-1.0" => SpdxLicense::Pddl1,
            "PHP-3.0" => SpdxLicense::Php3,
            "PHP-3.01" => SpdxLicense::Php31,
            "Plexus" => SpdxLicense::Plexus,
            "PostgreSQL" => SpdxLicense::PostgreSql,
            "psfrag" => SpdxLicense::Psfrag,
            "psutils" => SpdxLicense::Psutils,
            "Python-2.0" => SpdxLicense::Python2,
            "Qhull" => SpdxLicense::Qhull,
            "QPL-1.0" => SpdxLicense::Qpl1,
            "Rdisc" => SpdxLicense::Rdisc,
            "RHeCos-1.1" => SpdxLicense::RheCos1_1,
            "RPL-1.1" => SpdxLicense::Rpl1_1,
            "RPL-1.5" => SpdxLicense::Rpl1_5,
            "RPSL-1.0" => SpdxLicense::Rpsl1,
            "RSA-MD" => SpdxLicense::RsaMd,
            "RSCPL" => SpdxLicense::Rscpl,
            "Ruby" => SpdxLicense::Ruby,
            "SAX-PD" => SpdxLicense::SaxPd,
            "Saxpath" => SpdxLicense::Saxpath,
            "SCEA" => SpdxLicense::Scea,
            "Sendmail" => SpdxLicense::Sendmail,
            "Sendmail-8.23" => SpdxLicense::Sendmail8_23,
            "SGI-B-1.0" => SpdxLicense::SgiB1,
            "SGI-B-1.1" => SpdxLicense::SgiB1_1,
            "SGI-B-2.0" => SpdxLicense::SgiB2,
            "SHL-0.5" => SpdxLicense::Shl0_5,
            "SHL-0.51" => SpdxLicense::Shl0_51,
            "SimPL-2.0" => SpdxLicense::SimPl2,
            "SISSL" => SpdxLicense::Sissl,
            "SISSL-1.2" => SpdxLicense::Sissl1_2,
            "Sleepycat" => SpdxLicense::Sleepycat,
            "SMLNJ" => SpdxLicense::Smlnj,
            "SMPPL" => SpdxLicense::Smppl,
            "SNIA" => SpdxLicense::Snia,
            "Spencer-86" => SpdxLicense::Spencer86,
            "Spencer-94" => SpdxLicense::Spencer94,
            "Spencer-99" => SpdxLicense::Spencer99,
            "SPL-1.0" => SpdxLicense::Spl1,
            "SSH-OpenSSH" => SpdxLicense::SshOpenSsh,
            "SSH-short" => SpdxLicense::SshShort,
            "SSPL-1.0" => SpdxLicense::Sspl1,
            "StandardML-NJ" => SpdxLicense::StandardMlNj,
            "SugarCRM-1.1.3" => SpdxLicense::SugarCrm1_1_3,
            "SWL" => SpdxLicense::Swl,
            "TAPR-OHL-1.0" => SpdxLicense::TaprOhl1,
            "TCL" => SpdxLicense::Tcl,
            "TCP-wrappers" => SpdxLicense::TcpWrappers,
            "TMate" => SpdxLicense::Tmate,
            "TORQUE-1.1" => SpdxLicense::Torque1_1,
            "TOSL" => SpdxLicense::Tosl,
            "TU-Berlin-1.0" => SpdxLicense::TuBerlin1,
            "TU-Berlin-2.0" => SpdxLicense::TuBerlin2,
            "UCL-1.0" => SpdxLicense::Ucl1,
            "Unicode-DFS-2015" => SpdxLicense::UnicodeDfs2015,
            "Unicode-DFS-2016" => SpdxLicense::UnicodeDfs2016,
            "Unicode-TOU" => SpdxLicense::UnicodeTou,
            "Unlicense" => SpdxLicense::Unlicense,
            "UPL-1.0" => SpdxLicense::Upl1,
            "Vim" => SpdxLicense::Vim,
            "VOSTROM" => SpdxLicense::Vostrom,
            "VSL-1.0" => SpdxLicense::Vsl1,
            "W3C" => SpdxLicense::W3C,
            "W3C-19980720" => SpdxLicense::W3C19980720,
            "W3C-20150513" => SpdxLicense::W3C20150513,
            "Watcom-1.0" => SpdxLicense::Watcom1,
            "Wsuipa" => SpdxLicense::Wsuipa,
            "WTFPL" => SpdxLicense::Wtfpl,
            "wxWindows" => SpdxLicense::WxWindows,
            "X11" => SpdxLicense::X11,
            "Xerox" => SpdxLicense::Xerox,
            "XFree86-1.1" => SpdxLicense::Xfree86_1_1,
            "xinetd" => SpdxLicense::Xinetd,
            "Xnet" => SpdxLicense::Xnet,
            "xpp" => SpdxLicense::Xpp,
            "XSkat" => SpdxLicense::Xskat,
            "YPL-1.0" => SpdxLicense::Ypl1,
            "YPL-1.1" => SpdxLicense::Ypl1_1,
            "Zed" => SpdxLicense::Zed,
            "Zend-2.0" => SpdxLicense::Zend2,
            "Zimbra-1.3" => SpdxLicense::Zimbra1_3,
            "Zimbra-1.4" => SpdxLicense::Zimbra1_4,
            "Zlib" => SpdxLicense::Zlib,
            "zlib-acknowledgement" => SpdxLicense::ZlibAcknowledgement,
            "ZPL-1.1" => SpdxLicense::Zpl1_1,
            "ZPL-2.0" => SpdxLicense::Zpl2,
            "ZPL-2.1" => SpdxLicense::Zpl2_1,

            };

            ID_TO_LICENSE.get(id).map(|&l| l)
        }

        #[cfg(not(feature = "phf"))]
        // This wrapping trick allows for reusing `id_to_license`.
        Some(match id {
            "AAL" => SpdxLicense::Aal,
            "Abstyles" => SpdxLicense::Abstyles,
            "Adobe-2006" => SpdxLicense::Adobe2006,
            "Adobe-Glyph" => SpdxLicense::AdobeGlyph,
            "ADSL" => SpdxLicense::Adsl,
            "AFL-1.1" => SpdxLicense::Afl1_1,
            "AFL-1.2" => SpdxLicense::Afl1_2,
            "AFL-2.0" => SpdxLicense::Afl2,
            "AFL-2.1" => SpdxLicense::Afl2_1,
            "AFL-3.0" => SpdxLicense::Afl3,
            "Afmparse" => SpdxLicense::Afmparse,
            "AGPL-1.0" => SpdxLicense::Agpl1,
            "AGPL-1.0-or-later" => SpdxLicense::Agpl1r,
            "AGPL-1.0-only" => SpdxLicense::Agpl1y,
            "AGPL-3.0" => SpdxLicense::Agpl3,
            "AGPL-3.0-or-later" => SpdxLicense::Agpl3r,
            "AGPL-3.0-only" => SpdxLicense::Agpl3y,
            "Aladdin" => SpdxLicense::Aladdin,
            "AMDPLPA" => SpdxLicense::Amdplpa,
            "AML" => SpdxLicense::Aml,
            "AMPAS" => SpdxLicense::Ampas,
            "ANTLR-PD" => SpdxLicense::AntlrPd,
            "Apache-1.0" => SpdxLicense::Apache1,
            "Apache-1.1" => SpdxLicense::Apache1_1,
            "Apache-2.0" => SpdxLicense::Apache2,
            "APAFML" => SpdxLicense::Apafml,
            "APL-1.0" => SpdxLicense::Apl1,
            "APSL-1.0" => SpdxLicense::Apsl1,
            "APSL-1.1" => SpdxLicense::Apsl1_1,
            "APSL-1.2" => SpdxLicense::Apsl1_2,
            "APSL-2.0" => SpdxLicense::Apsl2,
            "Artistic-1.0" => SpdxLicense::Artistic1,
            "Artistic-1.0-Perl" => SpdxLicense::Artistic1l,
            "Artistic-1.0-cl8" => SpdxLicense::Artistic1l8,
            "Artistic-2.0" => SpdxLicense::Artistic2,
            "Bahyph" => SpdxLicense::Bahyph,
            "Barr" => SpdxLicense::Barr,
            "Beerware" => SpdxLicense::Beerware,
            "BitTorrent-1.0" => SpdxLicense::BitTorrent1,
            "BitTorrent-1.1" => SpdxLicense::BitTorrent1_1,
            "blessing" => SpdxLicense::Blessing,
            "BlueOak-1.0.0" => SpdxLicense::BlueOak1,
            "Borceux" => SpdxLicense::Borceux,
            "0BSD" => SpdxLicense::Bsd0,
            "BSD-1-Clause" => SpdxLicense::Bsd1Clause,
            "BSD-2-Clause" => SpdxLicense::Bsd2Clause,
            "BSD-2-Clause-FreeBSD" => SpdxLicense::Bsd2ClauseFreeBsd,
            "BSD-2-Clause-NetBSD" => SpdxLicense::Bsd2ClauseNetBsd,
            "BSD-2-Clause-Patent" => SpdxLicense::Bsd2ClausePatent,
            "BSD-3-Clause" => SpdxLicense::Bsd3Clause,
            "BSD-3-Clause-Attribution" => SpdxLicense::Bsd3ClauseAttribution,
            "BSD-3-Clause-Clear" => SpdxLicense::Bsd3ClauseClear,
            "BSD-3-Clause-LBNL" => SpdxLicense::Bsd3ClauseLbnl,
            "BSD-3-Clause-No-Nuclear-License" => SpdxLicense::Bsd3ClauseNoNuclearLicense,
            "BSD-3-Clause-No-Nuclear-License-2014" => SpdxLicense::Bsd3ClauseNoNuclearLicense2014,
            "BSD-3-Clause-No-Nuclear-Warranty" => SpdxLicense::Bsd3ClauseNoNuclearWarranty,
            "BSD-3-Clause-Open-MPI" => SpdxLicense::Bsd3ClauseOpenMpi,
            "BSD-4-Clause" => SpdxLicense::Bsd4Clause,
            "BSD-4-Clause-UC" => SpdxLicense::Bsd4ClauseUc,
            "BSD-Protection" => SpdxLicense::BsdProtection,
            "BSD-Source-Code" => SpdxLicense::BsdSourceCode,
            "BSL-1.0" => SpdxLicense::Bsl1,
            "bzip2-1.0.5" => SpdxLicense::Bzip2_1_5,
            "bzip2-1.0.6" => SpdxLicense::Bzip2_1_6,
            "Caldera" => SpdxLicense::Caldera,
            "CATOSL-1.1" => SpdxLicense::Catosl1_1,
            "CC0-1.0" => SpdxLicense::Cc0_1,
            "CC-BY-1.0" => SpdxLicense::CcBy1,
            "CC-BY-2.0" => SpdxLicense::CcBy2,
            "CC-BY-2.5" => SpdxLicense::CcBy2_5,
            "CC-BY-3.0" => SpdxLicense::CcBy3,
            "CC-BY-4.0" => SpdxLicense::CcBy4,
            "CC-BY-NC-1.0" => SpdxLicense::CcByNc1,
            "CC-BY-NC-2.0" => SpdxLicense::CcByNc2,
            "CC-BY-NC-2.5" => SpdxLicense::CcByNc2_5,
            "CC-BY-NC-3.0" => SpdxLicense::CcByNc3,
            "CC-BY-NC-4.0" => SpdxLicense::CcByNc4,
            "CC-BY-NC-ND-1.0" => SpdxLicense::CcByNcNd1,
            "CC-BY-NC-ND-2.0" => SpdxLicense::CcByNcNd2,
            "CC-BY-NC-ND-2.5" => SpdxLicense::CcByNcNd2_5,
            "CC-BY-NC-ND-3.0" => SpdxLicense::CcByNcNd3,
            "CC-BY-NC-ND-4.0" => SpdxLicense::CcByNcNd4,
            "CC-BY-NC-SA-1.0" => SpdxLicense::CcByNcSa1,
            "CC-BY-NC-SA-2.0" => SpdxLicense::CcByNcSa2,
            "CC-BY-NC-SA-2.5" => SpdxLicense::CcByNcSa2_5,
            "CC-BY-NC-SA-3.0" => SpdxLicense::CcByNcSa3,
            "CC-BY-NC-SA-4.0" => SpdxLicense::CcByNcSa4,
            "CC-BY-ND-1.0" => SpdxLicense::CcByNd1,
            "CC-BY-ND-2.0" => SpdxLicense::CcByNd2,
            "CC-BY-ND-2.5" => SpdxLicense::CcByNd2_5,
            "CC-BY-ND-3.0" => SpdxLicense::CcByNd3,
            "CC-BY-ND-4.0" => SpdxLicense::CcByNd4,
            "CC-BY-SA-1.0" => SpdxLicense::CcBySa1,
            "CC-BY-SA-2.0" => SpdxLicense::CcBySa2,
            "CC-BY-SA-2.5" => SpdxLicense::CcBySa2_5,
            "CC-BY-SA-3.0" => SpdxLicense::CcBySa3,
            "CC-BY-SA-4.0" => SpdxLicense::CcBySa4,
            "CC-PDDC" => SpdxLicense::CcPddc,
            "CDDL-1.0" => SpdxLicense::Cddl1,
            "CDDL-1.1" => SpdxLicense::Cddl1_1,
            "CDLA-Permissive-1.0" => SpdxLicense::CdlaPermissive1,
            "CDLA-Sharing-1.0" => SpdxLicense::CdlaSharing1,
            "CECILL-1.0" => SpdxLicense::Cecill1,
            "CECILL-1.1" => SpdxLicense::Cecill1_1,
            "CECILL-2.0" => SpdxLicense::Cecill2,
            "CECILL-2.1" => SpdxLicense::Cecill2_1,
            "CECILL-B" => SpdxLicense::CecillB,
            "CECILL-C" => SpdxLicense::CecillC,
            "CERN-OHL-1.1" => SpdxLicense::CernOhl1_1,
            "CERN-OHL-1.2" => SpdxLicense::CernOhl1_2,
            "ClArtistic" => SpdxLicense::ClArtistic,
            "CNRI-Jython" => SpdxLicense::CnriJython,
            "CNRI-Python" => SpdxLicense::CnriPython,
            "CNRI-Python-GPL-Compatible" => SpdxLicense::CnriPythonGplCompatible,
            "Condor-1.1" => SpdxLicense::Condor1_1,
            "copyleft-next-0.3.0" => SpdxLicense::CopyleftNext0_3,
            "copyleft-next-0.3.1" => SpdxLicense::CopyleftNext0_3_1,
            "CPAL-1.0" => SpdxLicense::Cpal1,
            "CPL-1.0" => SpdxLicense::Cpl1,
            "CPOL-1.02" => SpdxLicense::Cpol12,
            "Crossword" => SpdxLicense::Crossword,
            "CrystalStacker" => SpdxLicense::CrystalStacker,
            "CUA-OPL-1.0" => SpdxLicense::CuaOpl1,
            "Cube" => SpdxLicense::Cube,
            "curl" => SpdxLicense::Curl,
            "D-FSL-1.0" => SpdxLicense::DFsl1,
            "diffmark" => SpdxLicense::Diffmark,
            "DOC" => SpdxLicense::Doc,
            "Dotseqn" => SpdxLicense::Dotseqn,
            "DSDP" => SpdxLicense::Dsdp,
            "dvipdfm" => SpdxLicense::Dvipdfm,
            "eCos-2.0" => SpdxLicense::ECos2,
            "eGenix" => SpdxLicense::EGenix,
            "ECL-1.0" => SpdxLicense::Ecl1,
            "ECL-2.0" => SpdxLicense::Ecl2,
            "EFL-1.0" => SpdxLicense::Efl1,
            "EFL-2.0" => SpdxLicense::Efl2,
            "Entessa" => SpdxLicense::Entessa,
            "EPL-1.0" => SpdxLicense::Epl1,
            "EPL-2.0" => SpdxLicense::Epl2,
            "ErlPL-1.1" => SpdxLicense::ErlPl1_1,
            "etalab-2.0" => SpdxLicense::Etalab2,
            "EUDatagrid" => SpdxLicense::Eudatagrid,
            "EUPL-1.0" => SpdxLicense::Eupl1,
            "EUPL-1.1" => SpdxLicense::Eupl1_1,
            "EUPL-1.2" => SpdxLicense::Eupl1_2,
            "Eurosym" => SpdxLicense::Eurosym,
            "Fair" => SpdxLicense::Fair,
            "Frameworx-1.0" => SpdxLicense::Frameworx1,
            "FreeImage" => SpdxLicense::FreeImage,
            "FSFAP" => SpdxLicense::Fsfap,
            "FSFUL" => SpdxLicense::Fsful,
            "FSFULLR" => SpdxLicense::Fsfullr,
            "FTL" => SpdxLicense::Ftl,
            "gSOAP-1.3b" => SpdxLicense::GSoap1_3b,
            "GFDL-1.1" => SpdxLicense::Gfdl1_1,
            "GFDL-1.1-only" => SpdxLicense::Gfdl1_1Only,
            "GFDL-1.1-or-later" => SpdxLicense::Gfdl1_1OrLater,
            "GFDL-1.2" => SpdxLicense::Gfdl1_2,
            "GFDL-1.2-only" => SpdxLicense::Gfdl1_2Only,
            "GFDL-1.2-or-later" => SpdxLicense::Gfdl1_2OrLater,
            "GFDL-1.3" => SpdxLicense::Gfdl1_3,
            "GFDL-1.3-only" => SpdxLicense::Gfdl1_3Only,
            "GFDL-1.3-or-later" => SpdxLicense::Gfdl1_3OrLater,
            "Giftware" => SpdxLicense::Giftware,
            "GL2PS" => SpdxLicense::Gl2Ps,
            "Glide" => SpdxLicense::Glide,
            "Glulxe" => SpdxLicense::Glulxe,
            "gnuplot" => SpdxLicense::Gnuplot,
            "GPL-1.0" => SpdxLicense::Gpl1,
            "GPL-1.0+" => SpdxLicense::Gpl1Plus,
            "GPL-1.0-or-later" => SpdxLicense::Gpl1r,
            "GPL-1.0-only" => SpdxLicense::Gpl1y,
            "GPL-2.0" => SpdxLicense::Gpl2,
            "GPL-2.0+" => SpdxLicense::Gpl2Plus,
            "GPL-2.0-with-autoconf-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-bison-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-classpath-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-font-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-with-GCC-exception" => SpdxLicense::Gpl2n,
            "GPL-2.0-or-later" => SpdxLicense::Gpl2r,
            "GPL-2.0-only" => SpdxLicense::Gpl2y,
            "GPL-3.0" => SpdxLicense::Gpl3,
            "GPL-3.0+" => SpdxLicense::Gpl3Plus,
            "GPL-3.0-with-GCC-exception" => SpdxLicense::Gpl3n,
            "GPL-3.0-with-autoconf-exception" => SpdxLicense::Gpl3n,
            "GPL-3.0-or-later" => SpdxLicense::Gpl3r,
            "GPL-3.0-only" => SpdxLicense::Gpl3y,
            "HaskellReport" => SpdxLicense::HaskellReport,
            "HPND" => SpdxLicense::Hpnd,
            "HPND-sell-variant" => SpdxLicense::HpndSellVariant,
            "iMatix" => SpdxLicense::IMatix,
            "IBM-pibs" => SpdxLicense::IbmPibs,
            "ICU" => SpdxLicense::Icu,
            "IJG" => SpdxLicense::Ijg,
            "ImageMagick" => SpdxLicense::ImageMagick,
            "Imlib2" => SpdxLicense::Imlib2,
            "Info-ZIP" => SpdxLicense::InfoZip,
            "Intel" => SpdxLicense::Intel,
            "Intel-ACPI" => SpdxLicense::IntelAcpi,
            "Interbase-1.0" => SpdxLicense::Interbase1,
            "IPA" => SpdxLicense::Ipa,
            "IPL-1.0" => SpdxLicense::Ipl1,
            "ISC" => SpdxLicense::Isc,
            "JasPer-2.0" => SpdxLicense::JasPer2,
            "JPNIC" => SpdxLicense::Jpnic,
            "JSON" => SpdxLicense::Json,
            "LAL-1.2" => SpdxLicense::Lal1_2,
            "LAL-1.3" => SpdxLicense::Lal1_3,
            "Latex2e" => SpdxLicense::Latex2e,
            "Leptonica" => SpdxLicense::Leptonica,
            "LGPL-2.0" => SpdxLicense::Lgpl2,
            "LGPL-2.0+" => SpdxLicense::Lgpl2Plus,
            "LGPL-2.1" => SpdxLicense::Lgpl2_1,
            "LGPL-2.1-only" => SpdxLicense::Lgpl2_1Only,
            "LGPL-2.1-or-later" => SpdxLicense::Lgpl2_1OrLater,
            "LGPL-2.1+" => SpdxLicense::Lgpl2_1Plus,
            "LGPL-2.0-or-later" => SpdxLicense::Lgpl2r,
            "LGPL-2.0-only" => SpdxLicense::Lgpl2y,
            "LGPL-3.0" => SpdxLicense::Lgpl3,
            "LGPL-3.0+" => SpdxLicense::Lgpl3Plus,
            "LGPL-3.0-or-later" => SpdxLicense::Lgpl3r,
            "LGPL-3.0-only" => SpdxLicense::Lgpl3y,
            "LGPLLR" => SpdxLicense::Lgpllr,
            "LiLiQ-P-1.1" => SpdxLicense::LiLiQP1_1,
            "LiLiQ-R-1.1" => SpdxLicense::LiLiQR1_1,
            "LiLiQ-Rplus-1.1" => SpdxLicense::LiLiQRplus1_1,
            "Libpng" => SpdxLicense::Libpng,
            "libpng-2.0" => SpdxLicense::Libpng2,
            "libtiff" => SpdxLicense::Libtiff,
            "Linux-OpenIB" => SpdxLicense::LinuxOpenIb,
            "LPL-1.0" => SpdxLicense::Lpl1,
            "LPL-1.02" => SpdxLicense::Lpl12,
            "LPPL-1.0" => SpdxLicense::Lppl1,
            "LPPL-1.1" => SpdxLicense::Lppl1_1,
            "LPPL-1.2" => SpdxLicense::Lppl1_2,
            "LPPL-1.3a" => SpdxLicense::Lppl1_3a,
            "LPPL-1.3c" => SpdxLicense::Lppl1_3c,
            "MakeIndex" => SpdxLicense::MakeIndex,
            "MirOS" => SpdxLicense::MirOs,
            "MIT" => SpdxLicense::Mit,
            "MIT-0" => SpdxLicense::Mit0,
            "MIT-advertising" => SpdxLicense::MitAdvertising,
            "MIT-CMU" => SpdxLicense::MitCmu,
            "MIT-enna" => SpdxLicense::MitEnna,
            "MIT-feh" => SpdxLicense::MitFeh,
            "MITNFA" => SpdxLicense::Mitnfa,
            "Motosoto" => SpdxLicense::Motosoto,
            "mpich2" => SpdxLicense::Mpich2,
            "MPL-1.0" => SpdxLicense::Mpl1,
            "MPL-1.1" => SpdxLicense::Mpl1_1,
            "MPL-2.0" => SpdxLicense::Mpl2,
            "MPL-2.0-no-copyleft-exception" => SpdxLicense::Mpl2n,
            "MS-PL" => SpdxLicense::MsPl,
            "MS-RL" => SpdxLicense::MsRl,
            "MTLL" => SpdxLicense::Mtll,
            "MulanPSL-1.0" => SpdxLicense::MulanPsl1,
            "Multics" => SpdxLicense::Multics,
            "Mup" => SpdxLicense::Mup,
            "NASA-1.3" => SpdxLicense::Nasa1_3,
            "Naumen" => SpdxLicense::Naumen,
            "NBPL-1.0" => SpdxLicense::Nbpl1,
            "NCSA" => SpdxLicense::Ncsa,
            "NetCDF" => SpdxLicense::NetCdf,
            "Net-SNMP" => SpdxLicense::NetSnmp,
            "Newsletr" => SpdxLicense::Newsletr,
            "NGPL" => SpdxLicense::Ngpl,
            "NLOD-1.0" => SpdxLicense::Nlod1,
            "NLPL" => SpdxLicense::Nlpl,
            "Nokia" => SpdxLicense::Nokia,
            "NOSL" => SpdxLicense::Nosl,
            "Noweb" => SpdxLicense::Noweb,
            "NPL-1.0" => SpdxLicense::Npl1,
            "NPL-1.1" => SpdxLicense::Npl1_1,
            "NPOSL-3.0" => SpdxLicense::Nposl3,
            "NRL" => SpdxLicense::Nrl,
            "NTP" => SpdxLicense::Ntp,
            "Nunit" => SpdxLicense::Nunit,
            "OCCT-PL" => SpdxLicense::OcctPl,
            "OCLC-2.0" => SpdxLicense::Oclc2,
            "ODbL-1.0" => SpdxLicense::OdbL1,
            "ODC-By-1.0" => SpdxLicense::OdcBy1,
            "OFL-1.0" => SpdxLicense::Ofl1,
            "OFL-1.1" => SpdxLicense::Ofl1_1,
            "OGL-Canada-2.0" => SpdxLicense::OglCanada2,
            "OGL-UK-1.0" => SpdxLicense::OglUk1,
            "OGL-UK-2.0" => SpdxLicense::OglUk2,
            "OGL-UK-3.0" => SpdxLicense::OglUk3,
            "OGTSL" => SpdxLicense::Ogtsl,
            "OLDAP-1.1" => SpdxLicense::Oldap1_1,
            "OLDAP-1.2" => SpdxLicense::Oldap1_2,
            "OLDAP-1.3" => SpdxLicense::Oldap1_3,
            "OLDAP-1.4" => SpdxLicense::Oldap1_4,
            "OLDAP-2.0" => SpdxLicense::Oldap2,
            "OLDAP-2.0.1" => SpdxLicense::Oldap2_1,
            "OLDAP-2.1" => SpdxLicense::Oldap2_1,
            "OLDAP-2.2" => SpdxLicense::Oldap2_2,
            "OLDAP-2.2.1" => SpdxLicense::Oldap2_2_1,
            "OLDAP-2.2.2" => SpdxLicense::Oldap2_2_2,
            "OLDAP-2.3" => SpdxLicense::Oldap2_3,
            "OLDAP-2.4" => SpdxLicense::Oldap2_4,
            "OLDAP-2.5" => SpdxLicense::Oldap2_5,
            "OLDAP-2.6" => SpdxLicense::Oldap2_6,
            "OLDAP-2.7" => SpdxLicense::Oldap2_7,
            "OLDAP-2.8" => SpdxLicense::Oldap2_8,
            "OML" => SpdxLicense::Oml,
            "OpenSSL" => SpdxLicense::OpenSsl,
            "OPL-1.0" => SpdxLicense::Opl1,
            "OSET-PL-2.1" => SpdxLicense::OsetPl2_1,
            "OSL-1.0" => SpdxLicense::Osl1,
            "OSL-1.1" => SpdxLicense::Osl1_1,
            "OSL-2.0" => SpdxLicense::Osl2,
            "OSL-2.1" => SpdxLicense::Osl2_1,
            "OSL-3.0" => SpdxLicense::Osl3,
            "Parity-6.0.0" => SpdxLicense::Parity6,
            "PDDL-1.0" => SpdxLicense::Pddl1,
            "PHP-3.0" => SpdxLicense::Php3,
            "PHP-3.01" => SpdxLicense::Php31,
            "Plexus" => SpdxLicense::Plexus,
            "PostgreSQL" => SpdxLicense::PostgreSql,
            "psfrag" => SpdxLicense::Psfrag,
            "psutils" => SpdxLicense::Psutils,
            "Python-2.0" => SpdxLicense::Python2,
            "Qhull" => SpdxLicense::Qhull,
            "QPL-1.0" => SpdxLicense::Qpl1,
            "Rdisc" => SpdxLicense::Rdisc,
            "RHeCos-1.1" => SpdxLicense::RheCos1_1,
            "RPL-1.1" => SpdxLicense::Rpl1_1,
            "RPL-1.5" => SpdxLicense::Rpl1_5,
            "RPSL-1.0" => SpdxLicense::Rpsl1,
            "RSA-MD" => SpdxLicense::RsaMd,
            "RSCPL" => SpdxLicense::Rscpl,
            "Ruby" => SpdxLicense::Ruby,
            "SAX-PD" => SpdxLicense::SaxPd,
            "Saxpath" => SpdxLicense::Saxpath,
            "SCEA" => SpdxLicense::Scea,
            "Sendmail" => SpdxLicense::Sendmail,
            "Sendmail-8.23" => SpdxLicense::Sendmail8_23,
            "SGI-B-1.0" => SpdxLicense::SgiB1,
            "SGI-B-1.1" => SpdxLicense::SgiB1_1,
            "SGI-B-2.0" => SpdxLicense::SgiB2,
            "SHL-0.5" => SpdxLicense::Shl0_5,
            "SHL-0.51" => SpdxLicense::Shl0_51,
            "SimPL-2.0" => SpdxLicense::SimPl2,
            "SISSL" => SpdxLicense::Sissl,
            "SISSL-1.2" => SpdxLicense::Sissl1_2,
            "Sleepycat" => SpdxLicense::Sleepycat,
            "SMLNJ" => SpdxLicense::Smlnj,
            "SMPPL" => SpdxLicense::Smppl,
            "SNIA" => SpdxLicense::Snia,
            "Spencer-86" => SpdxLicense::Spencer86,
            "Spencer-94" => SpdxLicense::Spencer94,
            "Spencer-99" => SpdxLicense::Spencer99,
            "SPL-1.0" => SpdxLicense::Spl1,
            "SSH-OpenSSH" => SpdxLicense::SshOpenSsh,
            "SSH-short" => SpdxLicense::SshShort,
            "SSPL-1.0" => SpdxLicense::Sspl1,
            "StandardML-NJ" => SpdxLicense::StandardMlNj,
            "SugarCRM-1.1.3" => SpdxLicense::SugarCrm1_1_3,
            "SWL" => SpdxLicense::Swl,
            "TAPR-OHL-1.0" => SpdxLicense::TaprOhl1,
            "TCL" => SpdxLicense::Tcl,
            "TCP-wrappers" => SpdxLicense::TcpWrappers,
            "TMate" => SpdxLicense::Tmate,
            "TORQUE-1.1" => SpdxLicense::Torque1_1,
            "TOSL" => SpdxLicense::Tosl,
            "TU-Berlin-1.0" => SpdxLicense::TuBerlin1,
            "TU-Berlin-2.0" => SpdxLicense::TuBerlin2,
            "UCL-1.0" => SpdxLicense::Ucl1,
            "Unicode-DFS-2015" => SpdxLicense::UnicodeDfs2015,
            "Unicode-DFS-2016" => SpdxLicense::UnicodeDfs2016,
            "Unicode-TOU" => SpdxLicense::UnicodeTou,
            "Unlicense" => SpdxLicense::Unlicense,
            "UPL-1.0" => SpdxLicense::Upl1,
            "Vim" => SpdxLicense::Vim,
            "VOSTROM" => SpdxLicense::Vostrom,
            "VSL-1.0" => SpdxLicense::Vsl1,
            "W3C" => SpdxLicense::W3C,
            "W3C-19980720" => SpdxLicense::W3C19980720,
            "W3C-20150513" => SpdxLicense::W3C20150513,
            "Watcom-1.0" => SpdxLicense::Watcom1,
            "Wsuipa" => SpdxLicense::Wsuipa,
            "WTFPL" => SpdxLicense::Wtfpl,
            "wxWindows" => SpdxLicense::WxWindows,
            "X11" => SpdxLicense::X11,
            "Xerox" => SpdxLicense::Xerox,
            "XFree86-1.1" => SpdxLicense::Xfree86_1_1,
            "xinetd" => SpdxLicense::Xinetd,
            "Xnet" => SpdxLicense::Xnet,
            "xpp" => SpdxLicense::Xpp,
            "XSkat" => SpdxLicense::Xskat,
            "YPL-1.0" => SpdxLicense::Ypl1,
            "YPL-1.1" => SpdxLicense::Ypl1_1,
            "Zed" => SpdxLicense::Zed,
            "Zend-2.0" => SpdxLicense::Zend2,
            "Zimbra-1.3" => SpdxLicense::Zimbra1_3,
            "Zimbra-1.4" => SpdxLicense::Zimbra1_4,
            "Zlib" => SpdxLicense::Zlib,
            "zlib-acknowledgement" => SpdxLicense::ZlibAcknowledgement,
            "ZPL-1.1" => SpdxLicense::Zpl1_1,
            "ZPL-2.0" => SpdxLicense::Zpl2,
            "ZPL-2.1" => SpdxLicense::Zpl2_1,

            _ => return None,
        })
    }
}
