import ports.*;

/**
 * @ClassName Arbitrage
 * @Description arbitrage system abstraction
 * @Author happyboy
 * @Date 2025/9/9 2:23
 * @Version 1.0
 */
public class Arbitrage {
    private Binance binance;
    private Hyperliquid hyperliquid;
    private Bitget bitget;
    private Coinbase coinbase;
    private Crypto crypto;
    private Gate gate;
    private HTX htx;
    private Kraken kraken;
    private Kucoin kucoin;
    private Lighter lighter;
    private Mexc mexc;
    private OKX okx;
    private Upbit upbit;

    public static Arbitrage init(){
       return new Arbitrage();
    }

    public static Arbitrage initByJson(){
        return new Arbitrage();
    }

    public static Arbitrage initByJsonFile(){
        return new Arbitrage();
    }

    public static Arbitrage initByXMLFile(){
        return new Arbitrage();
    }

    public Binance getBinance() {
        return binance;
    }

    public void setBinance(Binance binance) {
        this.binance = binance;
    }

    public Hyperliquid getHyperliquid() {
        return hyperliquid;
    }

    public void setHyperliquid(Hyperliquid hyperliquid) {
        this.hyperliquid = hyperliquid;
    }

    public Bitget getBitget() {
        return bitget;
    }

    public void setBitget(Bitget bitget) {
        this.bitget = bitget;
    }

    public Coinbase getCoinbase() {
        return coinbase;
    }

    public void setCoinbase(Coinbase coinbase) {
        this.coinbase = coinbase;
    }

    public Crypto getCrypto() {
        return crypto;
    }

    public void setCrypto(Crypto crypto) {
        this.crypto = crypto;
    }

    public Gate getGate() {
        return gate;
    }

    public void setGate(Gate gate) {
        this.gate = gate;
    }

    public HTX getHtx() {
        return htx;
    }

    public void setHtx(HTX htx) {
        this.htx = htx;
    }

    public Kraken getKraken() {
        return kraken;
    }

    public void setKraken(Kraken kraken) {
        this.kraken = kraken;
    }

    public Kucoin getKucoin() {
        return kucoin;
    }

    public void setKucoin(Kucoin kucoin) {
        this.kucoin = kucoin;
    }

    public Lighter getLighter() {
        return lighter;
    }

    public void setLighter(Lighter lighter) {
        this.lighter = lighter;
    }

    public Mexc getMexc() {
        return mexc;
    }

    public void setMexc(Mexc mexc) {
        this.mexc = mexc;
    }

    public OKX getOkx() {
        return okx;
    }

    public void setOkx(OKX okx) {
        this.okx = okx;
    }

    public Upbit getUpbit() {
        return upbit;
    }

    public void setUpbit(Upbit upbit) {
        this.upbit = upbit;
    }
}
